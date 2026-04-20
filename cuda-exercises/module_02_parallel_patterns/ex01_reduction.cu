// EXERCISE 1: Add __syncthreads() inside the tree-reduction loop.
//
// block_sum reduces one block of 256 floats to a single sum using a
// halving-stride tree in shared memory. Each iteration, active threads
// add the value from stride positions away, then the stride halves.
//
// Bug: without __syncthreads() between iterations, threads at one level
// can read shared memory values that haven't been updated by the previous
// level yet. The partially-updated sdata produces a wrong sum.
//
// Fix: add __syncthreads() inside the for loop, after the addition.
//
// Build & run:
//   cmake --build build --target ex01_reduction && ./build/module_02_parallel_patterns/ex01_reduction

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

static const int BLOCK = 256;

__global__ void block_sum(const float* in, float* out, int n) {
    extern __shared__ float sdata[];
    int tid = threadIdx.x;
    int idx = blockIdx.x * blockDim.x + tid;

    sdata[tid] = (idx < n) ? in[idx] : 0.0f;
    __syncthreads();

    for (int s = BLOCK / 2; s > 0; s >>= 1) {
        if (tid < s) {
            sdata[tid] += sdata[tid + s];
        }
        // BUG: missing __syncthreads() here
        // Without it, faster threads race into the next iteration and read
        // values that haven't been updated by the current iteration yet.
    }

    if (tid == 0) out[blockIdx.x] = sdata[0];
}

int main() {
    const int NBLOCKS = 8;
    const int N = NBLOCKS * BLOCK;

    float h_in[N], h_out[NBLOCKS];
    for (int i = 0; i < N; i++) h_in[i] = 1.0f;

    float *d_in, *d_out;
    CUDA_CHECK(cudaMalloc(&d_in,  N       * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_out, NBLOCKS * sizeof(float)));
    CUDA_CHECK(cudaMemcpy(d_in, h_in, N * sizeof(float), cudaMemcpyHostToDevice));

    block_sum<<<NBLOCKS, BLOCK, BLOCK * sizeof(float)>>>(d_in, d_out, N);
    CUDA_CHECK(cudaGetLastError());
    CUDA_CHECK(cudaDeviceSynchronize());

    CUDA_CHECK(cudaMemcpy(h_out, d_out, NBLOCKS * sizeof(float), cudaMemcpyDeviceToHost));

    // Each block of BLOCK all-1 values should sum to exactly BLOCK
    bool all_ok = true;
    for (int b = 0; b < NBLOCKS; b++) {
        if (fabsf(h_out[b] - (float)BLOCK) > 0.5f) { all_ok = false; break; }
    }
    CHECK(all_ok);

    // Verify total
    float total = 0.0f;
    for (int b = 0; b < NBLOCKS; b++) total += h_out[b];
    CHECK(fabsf(total - (float)N) < 0.5f);

    cudaFree(d_in); cudaFree(d_out);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
