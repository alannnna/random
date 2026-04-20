// EXERCISE 3: Fix the XOR offset sequence in the warp-shuffle reduction.
//
// A warp reduction using __shfl_xor_sync exchanges values between lanes
// whose indices differ by a power of two, halving the active set each step.
// The correct sequence starts at the largest offset (16) and halves down to 1:
//   16 → 8 → 4 → 2 → 1
//
// Bug: the sequence starts at 1 and doubles (1 → 2 → 4 → 8 → 16).
// This is the Hillis-Steele pattern which requires log2 passes of work
// but also has data-dependency issues when used with XOR shuffle — later
// steps read values that were already combined in earlier steps, causing
// double-counting and a wrong (inflated) result.
//
// Fix: change the loop to start at offset=16 and halve each iteration.
//
// Build & run:
//   cmake --build build --target ex03_warp_reduce && ./build/module_04_warp_optimization/ex03_warp_reduce

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

__global__ void warp_sum(const float* in, float* out, int n) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    float val = (idx < n) ? in[idx] : 0.0f;

    // BUG: wrong order — starts at 1 and doubles instead of starting at 16 and halving
    for (int offset = 1; offset < 32; offset <<= 1) {
        val += __shfl_xor_sync(0xffffffff, val, offset);
    }

    // Lane 0 of each warp holds the warp's sum
    if ((threadIdx.x & 31) == 0) {
        atomicAdd(out, val);
    }
}

int main() {
    const int N = 256;  // exactly 8 warps of 32 threads

    float h_in[N], h_out_val;
    for (int i = 0; i < N; i++) h_in[i] = 1.0f;
    const float EXPECTED = (float)N;  // sum of 256 ones

    float *d_in, *d_out;
    CUDA_CHECK(cudaMalloc(&d_in,  N * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_out, sizeof(float)));
    CUDA_CHECK(cudaMemcpy(d_in, h_in, N * sizeof(float), cudaMemcpyHostToDevice));

    for (int trial = 0; trial < 5; trial++) {
        CUDA_CHECK(cudaMemset(d_out, 0, sizeof(float)));
        warp_sum<<<1, N>>>(d_in, d_out, N);
        CUDA_CHECK(cudaGetLastError());
        CUDA_CHECK(cudaDeviceSynchronize());

        CUDA_CHECK(cudaMemcpy(&h_out_val, d_out, sizeof(float), cudaMemcpyDeviceToHost));
        CHECK(fabsf(h_out_val - EXPECTED) < 0.5f);
    }

    cudaFree(d_in); cudaFree(d_out);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
