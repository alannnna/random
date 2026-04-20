// EXERCISE 5: Add __syncwarp() to synchronize threads within a warp.
//
// On Volta and later GPUs, threads within a warp are no longer guaranteed
// to execute in perfect lock-step (independent thread scheduling). Code
// that assumes intra-warp synchronization without an explicit barrier can
// silently produce wrong results.
//
// This kernel performs a warp-level prefix sum in shared memory:
//   step 1: each thread writes its value to sdata[lane]
//   step 2: each thread reads sdata[lane - offset] and adds it
// Between steps 1 and 2 we need all lanes to have committed their writes.
//
// Bug: __syncwarp() is missing after the write in step 1, so some lanes
// may read stale (pre-write) values from sdata, producing a wrong scan.
//
// Fix: add __syncwarp() after `sdata[lane] = val;` and after
// `sdata[lane] = val;` in the inner loop — or use the full mask form:
//   __syncwarp(0xffffffff);
//
// Build & run:
//   cmake --build build --target ex05_warp_sync && ./build/module_04_warp_optimization/ex05_warp_sync

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

// Warp-level inclusive prefix scan using shared memory
__global__ void warp_scan(const float* in, float* out, int n) {
    extern __shared__ float sdata[];
    int idx  = blockIdx.x * blockDim.x + threadIdx.x;
    int lane = threadIdx.x & 31;

    float val = (idx < n) ? in[idx] : 0.0f;
    sdata[threadIdx.x] = val;
    // BUG: missing __syncwarp() here
    // On Volta+, the lane that does the read below may run ahead of the
    // lane that should have written sdata[], producing a wrong prefix sum.

    for (int d = 1; d < 32; d <<= 1) {
        float addend = (lane >= d) ? sdata[threadIdx.x - d] : 0.0f;
        // BUG: missing __syncwarp() here before writing back
        sdata[threadIdx.x] = val = val + addend;
        // BUG: missing __syncwarp() here after writing back
    }

    if (idx < n) out[idx] = sdata[threadIdx.x];
}

int main() {
    // One warp (32 threads), input all 1.0f → inclusive scan = [1,2,3,...,32]
    const int N = 32;

    float h_in[N], h_out[N];
    for (int i = 0; i < N; i++) h_in[i] = 1.0f;

    float *d_in, *d_out;
    CUDA_CHECK(cudaMalloc(&d_in,  N * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_out, N * sizeof(float)));
    CUDA_CHECK(cudaMemcpy(d_in, h_in, N * sizeof(float), cudaMemcpyHostToDevice));

    warp_scan<<<1, N, N * sizeof(float)>>>(d_in, d_out, N);
    CUDA_CHECK(cudaGetLastError());
    CUDA_CHECK(cudaDeviceSynchronize());

    CUDA_CHECK(cudaMemcpy(h_out, d_out, N * sizeof(float), cudaMemcpyDeviceToHost));

    bool all_ok = true;
    for (int i = 0; i < N; i++) {
        if (fabsf(h_out[i] - (float)(i + 1)) > 0.5f) { all_ok = false; break; }
    }
    CHECK(all_ok);
    CHECK(fabsf(h_out[N - 1] - (float)N) < 0.5f);

    cudaFree(d_in); cudaFree(d_out);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
