// EXERCISE 3: Add __syncthreads() after storing into shared memory.
//
// square_sum computes the sum of squares of a block's elements. Each of
// the 256 threads stores val*val into shared memory, then thread 0 sums
// all 256 entries with a sequential loop.
//
// Bug: without __syncthreads(), thread 0 (warp 0) can race ahead and read
// shared[128..255] before warps 4-7 have written their values. The sum
// is wrong — typically much less than expected.
//
// Fix: add __syncthreads() after the store and before the sum loop.
//
// Build & run:
//   cmake --build build --target ex03_shared_sync && ./build/module_01_memory/ex03_shared_sync

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

static const int BLOCK = 256;

__global__ void square_sum(const float* in, float* out, int n) {
    extern __shared__ float sdata[];
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    int tid = threadIdx.x;

    sdata[tid] = (idx < n) ? in[idx] * in[idx] : 0.0f;
    // BUG: missing __syncthreads() here
    // Thread 0 starts summing before warps 4-7 have written sdata[128..255]

    if (tid == 0) {
        float sum = 0.0f;
        for (int j = 0; j < BLOCK; j++) sum += sdata[j];
        out[blockIdx.x] = sum;
    }
}

int main() {
    // 4 blocks of 256 threads each; input is all 1.0f
    // Expected: each block sums 256 squares of 1.0 = 256.0
    const int NBLOCKS = 4;
    const int N = NBLOCKS * BLOCK;

    float h_in[N], h_out[NBLOCKS];
    for (int i = 0; i < N; i++) h_in[i] = 1.0f;

    float *d_in, *d_out;
    CUDA_CHECK(cudaMalloc(&d_in,  N       * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_out, NBLOCKS * sizeof(float)));
    CUDA_CHECK(cudaMemcpy(d_in, h_in, N * sizeof(float), cudaMemcpyHostToDevice));

    square_sum<<<NBLOCKS, BLOCK, BLOCK * sizeof(float)>>>(d_in, d_out, N);
    CUDA_CHECK(cudaGetLastError());
    CUDA_CHECK(cudaDeviceSynchronize());

    CUDA_CHECK(cudaMemcpy(h_out, d_out, NBLOCKS * sizeof(float), cudaMemcpyDeviceToHost));

    bool all_ok = true;
    for (int b = 0; b < NBLOCKS; b++) {
        if (fabsf(h_out[b] - (float)BLOCK) > 0.5f) { all_ok = false; break; }
    }
    CHECK(all_ok);

    cudaFree(d_in); cudaFree(d_out);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
