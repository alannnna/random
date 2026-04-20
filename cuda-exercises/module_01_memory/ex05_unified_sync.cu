// EXERCISE 5: Add cudaDeviceSynchronize() before reading unified memory.
//
// cudaMallocManaged allocates memory visible to both GPU and CPU.
// After launching a kernel, the CPU must wait for the GPU to finish
// before reading results. Without the synchronization the CPU may read
// stale (pre-kernel) values because the kernel is still running.
//
// Fix: call cudaDeviceSynchronize() after the kernel launch and before
// reading data[0].
//
// Note: most reliably observed on discrete (non-integrated) GPUs.
//
// Build & run:
//   cmake --build build --target ex05_unified_sync && ./build/module_01_memory/ex05_unified_sync

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

// Kernel does a non-trivial amount of work so the CPU race is observable
__global__ void compute(float* data, int n) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= n) return;
    float v = (float)idx;
    // Enough arithmetic to keep the GPU busy for a measurable time
    for (int i = 0; i < 500; i++) v = sqrtf(v * v + 1.0f);
    data[idx] = v;
}

int main() {
    const int N = 1 << 20;  // 1M elements

    float* data;
    CUDA_CHECK(cudaMallocManaged(&data, N * sizeof(float)));

    // Initialise to a sentinel so a missing sync is detectable
    for (int i = 0; i < N; i++) data[i] = -1.0f;

    const int BLOCK = 256;
    const int GRID  = (N + BLOCK - 1) / BLOCK;

    compute<<<GRID, BLOCK>>>(data, N);
    CUDA_CHECK(cudaGetLastError());
    // BUG: missing cudaDeviceSynchronize() here
    // Reading data[] while the kernel is still running returns stale values.

    // data[0] should be a positive float computed by the kernel (not -1)
    CHECK(data[0] > 0.0f);
    // Spot-check a few more elements
    CHECK(data[N / 2] > 0.0f);
    CHECK(data[N - 1] > 0.0f);

    CUDA_CHECK(cudaFree(data));

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
