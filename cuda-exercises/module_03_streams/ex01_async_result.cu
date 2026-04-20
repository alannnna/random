// EXERCISE 1: Add cudaStreamSynchronize before reading the async result.
//
// cudaMemcpyAsync returns immediately on the host; the device-to-host
// copy runs in the background on the stream. Reading h_result before
// the copy is complete yields stale data (the initial sentinel value).
//
// Fix: call cudaStreamSynchronize(stream) after the cudaMemcpyAsync and
// before reading h_result[0].
//
// Note: most reliably observed on discrete GPUs with dedicated video memory.
//
// Build & run:
//   cmake --build build --target ex01_async_result && ./build/module_03_streams/ex01_async_result

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

__global__ void fill(float* data, float val, int n) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < n) data[idx] = val;
}

int main() {
    // Large array so the async D2H copy takes measurable time
    const int N = 8 * 1024 * 1024;  // 32 MB of floats
    const float EXPECTED = 42.0f;
    const float SENTINEL = -1.0f;

    cudaStream_t stream;
    CUDA_CHECK(cudaStreamCreate(&stream));

    float *d_data, *h_result;
    CUDA_CHECK(cudaMalloc(&d_data, N * sizeof(float)));
    // Pinned host memory is required for truly async transfers
    CUDA_CHECK(cudaMallocHost(&h_result, N * sizeof(float)));

    // Initialise host buffer to sentinel so a missing sync is detectable
    for (int i = 0; i < N; i++) h_result[i] = SENTINEL;

    // Fill device array, then async-copy back to host
    fill<<<(N + 255) / 256, 256, 0, stream>>>(d_data, EXPECTED, N);
    CUDA_CHECK(cudaGetLastError());
    cudaMemcpyAsync(h_result, d_data, N * sizeof(float), cudaMemcpyDeviceToHost, stream);
    // BUG: missing cudaStreamSynchronize(stream) here
    // The copy may still be in flight when we read h_result below.

    CHECK(h_result[0]     == EXPECTED);
    CHECK(h_result[N / 2] == EXPECTED);
    CHECK(h_result[N - 1] == EXPECTED);

    CUDA_CHECK(cudaStreamDestroy(stream));
    CUDA_CHECK(cudaFree(d_data));
    CUDA_CHECK(cudaFreeHost(h_result));

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
