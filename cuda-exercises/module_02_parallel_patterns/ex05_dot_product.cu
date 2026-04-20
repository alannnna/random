// EXERCISE 5: Fix the host-side partial-sum loop bound.
//
// The dot product is computed in two phases: each GPU block produces one
// partial sum stored in d_partial[], then the host loops over d_partial[]
// to compute the final scalar result.
//
// Bug: the host loop iterates N times (the length of the original arrays)
// instead of numBlocks times (the number of partial sums). It reads far
// past the end of h_partial[], returning garbage accumulated into the total.
//
// Fix: change `i < N` to `i < numBlocks` in the host reduction loop.
//
// Build & run:
//   cmake --build build --target ex05_dot_product && ./build/module_02_parallel_patterns/ex05_dot_product

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

static const int BLOCK = 256;

__global__ void partial_dot(const float* a, const float* b, float* partial, int n) {
    extern __shared__ float sdata[];
    int tid = threadIdx.x;
    int idx = blockIdx.x * blockDim.x + tid;

    sdata[tid] = (idx < n) ? a[idx] * b[idx] : 0.0f;
    __syncthreads();

    for (int s = BLOCK / 2; s > 0; s >>= 1) {
        if (tid < s) sdata[tid] += sdata[tid + s];
        __syncthreads();
    }

    if (tid == 0) partial[blockIdx.x] = sdata[0];
}

int main() {
    const int N        = 1024;
    const int numBlocks = (N + BLOCK - 1) / BLOCK;

    float h_a[N], h_b[N];
    for (int i = 0; i < N; i++) { h_a[i] = 1.0f; h_b[i] = 1.0f; }
    // dot(all-1, all-1) = N

    float *d_a, *d_b, *d_partial;
    CUDA_CHECK(cudaMalloc(&d_a,       N        * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_b,       N        * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_partial, numBlocks * sizeof(float)));
    CUDA_CHECK(cudaMemcpy(d_a, h_a, N * sizeof(float), cudaMemcpyHostToDevice));
    CUDA_CHECK(cudaMemcpy(d_b, h_b, N * sizeof(float), cudaMemcpyHostToDevice));

    partial_dot<<<numBlocks, BLOCK, BLOCK * sizeof(float)>>>(d_a, d_b, d_partial, N);
    CUDA_CHECK(cudaGetLastError());
    CUDA_CHECK(cudaDeviceSynchronize());

    float h_partial[numBlocks];
    CUDA_CHECK(cudaMemcpy(h_partial, d_partial, numBlocks * sizeof(float), cudaMemcpyDeviceToHost));

    // BUG: loop bound is N instead of numBlocks — reads garbage past h_partial[]
    float result = 0.0f;
    for (int i = 0; i < N; i++) result += h_partial[i];

    CHECK(fabsf(result - (float)N) < 0.5f);

    cudaFree(d_a); cudaFree(d_b); cudaFree(d_partial);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
