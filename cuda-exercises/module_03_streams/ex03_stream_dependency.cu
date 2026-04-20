// EXERCISE 3: Add an inter-stream dependency so the kernel waits for the copy.
//
// The host-to-device copy of d_in runs on stream1. The kernel that reads
// d_in runs on stream2. Because streams execute concurrently, the kernel
// can start before the copy finishes — reading zeros (the cudaMemset value)
// instead of the actual input data.
//
// Fix:
//   1. After the cudaMemcpyAsync on stream1, record an event:
//        cudaEventRecord(copy_done, stream1);
//   2. Before the kernel on stream2, insert a wait:
//        cudaStreamWaitEvent(stream2, copy_done, 0);
//
// Build & run:
//   cmake --build build --target ex03_stream_dependency && ./build/module_03_streams/ex03_stream_dependency

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

__global__ void scale(const float* in, float* out, float factor, int n) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < n) out[idx] = in[idx] * factor;
}

int main() {
    const int N      = 4 * 1024 * 1024;  // 16 MB — large enough that copy takes time
    const float FACTOR = 2.0f;

    cudaStream_t stream1, stream2;
    CUDA_CHECK(cudaStreamCreate(&stream1));
    CUDA_CHECK(cudaStreamCreate(&stream2));

    float *d_in, *d_out, *h_in, *h_out;
    CUDA_CHECK(cudaMallocHost(&h_in,  N * sizeof(float)));
    CUDA_CHECK(cudaMallocHost(&h_out, N * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_in,  N * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_out, N * sizeof(float)));

    for (int i = 0; i < N; i++) h_in[i] = 1.0f;
    CUDA_CHECK(cudaMemset(d_in, 0, N * sizeof(float)));  // initialize to 0

    // Copy input to device on stream1
    cudaMemcpyAsync(d_in, h_in, N * sizeof(float), cudaMemcpyHostToDevice, stream1);
    // BUG: no cudaStreamWaitEvent here — kernel on stream2 races with the copy on stream1
    // d_in may still be all zeros when scale() reads it

    scale<<<(N + 255) / 256, 256, 0, stream2>>>(d_in, d_out, FACTOR, N);
    CUDA_CHECK(cudaGetLastError());

    cudaMemcpyAsync(h_out, d_out, N * sizeof(float), cudaMemcpyDeviceToHost, stream2);
    CUDA_CHECK(cudaStreamSynchronize(stream2));

    // All outputs should be 1.0 * 2.0 = 2.0
    bool all_ok = true;
    for (int i = 0; i < N; i++) {
        if (fabsf(h_out[i] - FACTOR) > 0.01f) { all_ok = false; break; }
    }
    CHECK(all_ok);

    CUDA_CHECK(cudaStreamDestroy(stream1));
    CUDA_CHECK(cudaStreamDestroy(stream2));
    CUDA_CHECK(cudaFree(d_in)); CUDA_CHECK(cudaFree(d_out));
    CUDA_CHECK(cudaFreeHost(h_in)); CUDA_CHECK(cudaFreeHost(h_out));

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
