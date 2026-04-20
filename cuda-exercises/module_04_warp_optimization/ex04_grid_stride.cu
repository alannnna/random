// EXERCISE 4: Fix the stride in the grid-stride loop.
//
// A grid-stride loop lets a fixed-size grid process an arbitrarily large
// array by having each thread advance by the total number of threads
// (gridDim.x * blockDim.x) after each iteration.
//
// Bug: the stride is set to just blockDim.x. With a grid of 4 blocks:
//   - Thread 0 of block 0 processes elements 0, 256, 512, ...
//   - Thread 0 of block 1 processes elements 256, 512, ... (overlap!)
//   Elements in the overlap zone are negated twice (back to positive),
//   while elements at the end may be skipped.
//
// Fix: set stride = gridDim.x * blockDim.x.
//
// Build & run:
//   cmake --build build --target ex04_grid_stride && ./build/module_04_warp_optimization/ex04_grid_stride

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

__global__ void negate_all(float* data, int n) {
    int start = blockIdx.x * blockDim.x + threadIdx.x;
    // BUG: stride is blockDim.x — overlapping ranges between blocks
    int stride = blockDim.x;
    for (int i = start; i < n; i += stride) {
        data[i] = -data[i];
    }
}

int main() {
    const int N     = 2048;
    const int BLOCK = 256;
    const int GRID  = 4;  // far fewer than N/BLOCK — grid-stride loop needed

    float h_data[N], h_out[N];
    for (int i = 0; i < N; i++) h_data[i] = 1.0f;

    float* d_data;
    CUDA_CHECK(cudaMalloc(&d_data, N * sizeof(float)));
    CUDA_CHECK(cudaMemcpy(d_data, h_data, N * sizeof(float), cudaMemcpyHostToDevice));

    negate_all<<<GRID, BLOCK>>>(d_data, N);
    CUDA_CHECK(cudaGetLastError());
    CUDA_CHECK(cudaDeviceSynchronize());

    CUDA_CHECK(cudaMemcpy(h_out, d_data, N * sizeof(float), cudaMemcpyDeviceToHost));

    // Every element should be -1.0f after one negation
    bool all_ok = true;
    for (int i = 0; i < N; i++) {
        if (fabsf(h_out[i] - (-1.0f)) > 0.01f) { all_ok = false; break; }
    }
    CHECK(all_ok);
    CHECK(fabsf(h_out[0]     - (-1.0f)) < 0.01f);
    CHECK(fabsf(h_out[N - 1] - (-1.0f)) < 0.01f);

    cudaFree(d_data);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
