// EXERCISE 2: Move cudaEventRecord to the correct positions.
//
// cudaEventRecord inserts a timestamp marker into the GPU command queue at
// the point it is called. To measure how long a kernel takes, the start
// event must be recorded BEFORE the kernel launch and the stop event AFTER.
//
// Bug: both events are recorded before the kernel launch, so the elapsed
// time between them is ~0 ms regardless of how long the kernel runs.
//
// Fix: record `start` before the kernel launch and `stop` after it.
//
// Build & run:
//   cmake --build build --target ex02_event_timing && ./build/module_03_streams/ex02_event_timing

#include <cstdio>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

// Kernel that does enough work to take at least a few milliseconds
__global__ void busy_work(float* data, int n, int iters) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= n) return;
    float v = data[idx];
    for (int i = 0; i < iters; i++) v = sqrtf(v * v + 1.0f);
    data[idx] = v;
}

int main() {
    const int N     = 1 << 20;
    const int ITERS = 1000;

    float* d_data;
    CUDA_CHECK(cudaMalloc(&d_data, N * sizeof(float)));
    CUDA_CHECK(cudaMemset(d_data, 0, N * sizeof(float)));

    cudaEvent_t start, stop;
    CUDA_CHECK(cudaEventCreate(&start));
    CUDA_CHECK(cudaEventCreate(&stop));

    // BUG: both events recorded before the kernel — elapsed time will be ~0 ms
    CUDA_CHECK(cudaEventRecord(start));
    CUDA_CHECK(cudaEventRecord(stop));

    busy_work<<<(N + 255) / 256, 256>>>(d_data, N, ITERS);
    CUDA_CHECK(cudaGetLastError());

    CUDA_CHECK(cudaEventSynchronize(stop));

    float elapsed_ms = 0.0f;
    CUDA_CHECK(cudaEventElapsedTime(&elapsed_ms, start, stop));

    // The kernel should take well over 1 ms; elapsed ~0 means events were placed wrong
    CHECK(elapsed_ms > 1.0f);

    CUDA_CHECK(cudaEventDestroy(start));
    CUDA_CHECK(cudaEventDestroy(stop));
    CUDA_CHECK(cudaFree(d_data));

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
