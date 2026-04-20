// EXERCISE 4: Fix the wrong index in the 1-D stencil kernel.
//
// stencil computes out[i] = in[i-1] + in[i] + in[i+1] for interior elements.
// The kernel reads `in[i]` twice instead of `in[i-1]`, so the left neighbor
// is never included in the sum.
//
// Fix: change the first term from `in[i]` to `in[i - 1]`.
//
// Build & run:
//   cmake --build build --target ex04_stencil && ./build/module_02_parallel_patterns/ex04_stencil

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

__global__ void stencil(const float* in, float* out, int n) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i == 0 || i >= n - 1) return;
    // BUG: reads in[i] twice — should be in[i-1] + in[i] + in[i+1]
    out[i] = in[i] + in[i] + in[i + 1];
}

int main() {
    const int N = 1024;

    float h_in[N], h_out[N] = {};
    for (int i = 0; i < N; i++) h_in[i] = (float)i;

    float *d_in, *d_out;
    CUDA_CHECK(cudaMalloc(&d_in,  N * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_out, N * sizeof(float)));
    CUDA_CHECK(cudaMemcpy(d_in, h_in, N * sizeof(float), cudaMemcpyHostToDevice));
    CUDA_CHECK(cudaMemset(d_out, 0, N * sizeof(float)));

    stencil<<<4, 256>>>(d_in, d_out, N);
    CUDA_CHECK(cudaGetLastError());
    CUDA_CHECK(cudaDeviceSynchronize());

    CUDA_CHECK(cudaMemcpy(h_out, d_out, N * sizeof(float), cudaMemcpyDeviceToHost));

    // For interior elements: expected out[i] = (i-1) + i + (i+1) = 3*i
    bool all_ok = true;
    for (int i = 1; i < N - 1; i++) {
        float expected = (float)(i - 1) + (float)i + (float)(i + 1);
        if (fabsf(h_out[i] - expected) > 0.5f) { all_ok = false; break; }
    }
    CHECK(all_ok);
    // Spot-check a specific interior element
    CHECK(fabsf(h_out[100] - (99.0f + 100.0f + 101.0f)) < 0.5f);

    cudaFree(d_in); cudaFree(d_out);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
