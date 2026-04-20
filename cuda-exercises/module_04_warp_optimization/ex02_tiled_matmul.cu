// EXERCISE 2: Add __syncthreads() between the tile load and tile compute phases.
//
// Tiled matrix multiplication loads a TILE×TILE sub-block of A and B into
// shared memory, computes partial dot products, then moves to the next tile.
// Two __syncthreads() calls are needed per tile iteration:
//   1. After loading, before computing — ensures all threads see valid sdata.
//   2. After computing, before the next load — prevents overwriting sdata
//      while other threads are still reading from it.
//
// Bug: the first __syncthreads() (after load, before compute) is missing.
// Faster threads begin computing before all shared-memory slots are written,
// producing wrong partial products and a wrong final result.
//
// Fix: add __syncthreads() after the shared-memory load and before the
// inner dot-product loop.
//
// Build & run:
//   cmake --build build --target ex02_tiled_matmul && ./build/module_04_warp_optimization/ex02_tiled_matmul

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

static const int TILE = 16;
static const int M    = 32;  // square matrices M×M

__global__ void matmul(const float* A, const float* B, float* C, int m) {
    __shared__ float sA[TILE][TILE];
    __shared__ float sB[TILE][TILE];

    int row = blockIdx.y * TILE + threadIdx.y;
    int col = blockIdx.x * TILE + threadIdx.x;
    float sum = 0.0f;

    for (int t = 0; t * TILE < m; t++) {
        // Load tile from A and B into shared memory
        int aCol = t * TILE + threadIdx.x;
        int bRow = t * TILE + threadIdx.y;
        sA[threadIdx.y][threadIdx.x] = (row < m && aCol < m) ? A[row * m + aCol] : 0.0f;
        sB[threadIdx.y][threadIdx.x] = (bRow < m && col < m) ? B[bRow * m + col] : 0.0f;
        // BUG: missing __syncthreads() here
        // Some threads start the dot-product loop before other threads have
        // finished writing sA/sB, reading garbage values.

        for (int k = 0; k < TILE; k++) {
            sum += sA[threadIdx.y][k] * sB[k][threadIdx.x];
        }
        __syncthreads();  // wait before overwriting sA/sB in next iteration
    }

    if (row < m && col < m) C[row * m + col] = sum;
}

int main() {
    // A = identity, B = all-1s → C = all-1s
    float h_A[M * M] = {}, h_B[M * M], h_C[M * M];
    for (int i = 0; i < M; i++) h_A[i * M + i] = 1.0f;
    for (int i = 0; i < M * M; i++) h_B[i] = 1.0f;

    float *d_A, *d_B, *d_C;
    CUDA_CHECK(cudaMalloc(&d_A, M * M * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_B, M * M * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_C, M * M * sizeof(float)));
    CUDA_CHECK(cudaMemcpy(d_A, h_A, M * M * sizeof(float), cudaMemcpyHostToDevice));
    CUDA_CHECK(cudaMemcpy(d_B, h_B, M * M * sizeof(float), cudaMemcpyHostToDevice));

    dim3 block(TILE, TILE);
    dim3 grid((M + TILE - 1) / TILE, (M + TILE - 1) / TILE);
    matmul<<<grid, block>>>(d_A, d_B, d_C, M);
    CUDA_CHECK(cudaGetLastError());
    CUDA_CHECK(cudaDeviceSynchronize());

    CUDA_CHECK(cudaMemcpy(h_C, d_C, M * M * sizeof(float), cudaMemcpyDeviceToHost));

    bool all_ok = true;
    for (int i = 0; i < M * M; i++) {
        if (fabsf(h_C[i] - 1.0f) > 0.01f) { all_ok = false; break; }
    }
    CHECK(all_ok);

    cudaFree(d_A); cudaFree(d_B); cudaFree(d_C);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
