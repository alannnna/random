// EXERCISE 1: Fix the kernel launch grid dimension.
//
// vector_add adds two float arrays element-wise. The kernel is correct,
// but the grid is launched with N/BLOCK_SIZE blocks. Integer division
// truncates: for N=1000 and BLOCK_SIZE=256 that is 3 blocks (768 threads),
// leaving elements 768..999 with their initial value of 0.
//
// Fix: compute the grid as (N + BLOCK_SIZE - 1) / BLOCK_SIZE so the
// last partial block is always included.
//
// Build & run:
//   cmake --build build --target ex01_vector_add && ./build/module_01_memory/ex01_vector_add

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

static const int BLOCK_SIZE = 256;

__global__ void vector_add(const float* a, const float* b, float* c, int n) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < n) c[idx] = a[idx] + b[idx];
}

int main() {
    const int N = 1000;

    float h_a[N], h_b[N], h_c[N] = {};
    for (int i = 0; i < N; i++) { h_a[i] = (float)i; h_b[i] = (float)(N - i); }

    float *d_a, *d_b, *d_c;
    CUDA_CHECK(cudaMalloc(&d_a, N * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_b, N * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_c, N * sizeof(float)));
    CUDA_CHECK(cudaMemset(d_c, 0, N * sizeof(float)));
    CUDA_CHECK(cudaMemcpy(d_a, h_a, N * sizeof(float), cudaMemcpyHostToDevice));
    CUDA_CHECK(cudaMemcpy(d_b, h_b, N * sizeof(float), cudaMemcpyHostToDevice));

    // BUG: integer division truncates — the last partial block is missing
    int grid = N / BLOCK_SIZE;
    vector_add<<<grid, BLOCK_SIZE>>>(d_a, d_b, d_c, N);
    CUDA_CHECK(cudaGetLastError());
    CUDA_CHECK(cudaDeviceSynchronize());

    CUDA_CHECK(cudaMemcpy(h_c, d_c, N * sizeof(float), cudaMemcpyDeviceToHost));

    // All elements should equal N (a[i] + b[i] = i + (N-i) = N)
    bool all_ok = true;
    for (int i = 0; i < N; i++) {
        if (fabsf(h_c[i] - (float)N) > 0.5f) { all_ok = false; break; }
    }
    CHECK(all_ok);
    CHECK(fabsf(h_c[N - 1] - (float)N) < 0.5f);

    cudaFree(d_a); cudaFree(d_b); cudaFree(d_c);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
