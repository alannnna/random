// EXERCISE 5: Synchronize all streams before reading results.
//
// Four independent kernels run on four separate streams. After launching
// all of them, the host reads back each result. Only stream[0] is
// synchronized, so results from streams 1-3 may not be ready yet.
//
// Fix: synchronize every stream (or replace with cudaDeviceSynchronize())
// before reading any result.
//
// Build & run:
//   cmake --build build --target ex05_multi_stream_sync && ./build/module_03_streams/ex05_multi_stream_sync

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

// Kernel does real work so it takes measurable time
__global__ void compute_sum(const float* in, float* out, int n, int iters) {
    extern __shared__ float sdata[];
    int tid = threadIdx.x;
    int idx = blockIdx.x * blockDim.x + tid;

    float v = (idx < n) ? in[idx] : 0.0f;
    for (int i = 0; i < iters; i++) v = sqrtf(v + 1.0f);
    sdata[tid] = v;
    __syncthreads();

    for (int s = blockDim.x / 2; s > 0; s >>= 1) {
        if (tid < s) sdata[tid] += sdata[tid + s];
        __syncthreads();
    }
    if (tid == 0) atomicAdd(out, sdata[0]);
}

int main() {
    const int NSTREAMS = 4;
    const int N        = 1 << 18;
    const int BLOCK    = 256;
    const int ITERS    = 200;

    cudaStream_t streams[NSTREAMS];
    float *d_in[NSTREAMS], *d_out[NSTREAMS], *h_in[NSTREAMS], h_result[NSTREAMS];

    for (int s = 0; s < NSTREAMS; s++) {
        CUDA_CHECK(cudaStreamCreate(&streams[s]));
        CUDA_CHECK(cudaMallocHost(&h_in[s], N * sizeof(float)));
        CUDA_CHECK(cudaMalloc(&d_in[s],  N * sizeof(float)));
        CUDA_CHECK(cudaMalloc(&d_out[s], sizeof(float)));
        for (int i = 0; i < N; i++) h_in[s][i] = 1.0f;
        CUDA_CHECK(cudaMemcpy(d_in[s], h_in[s], N * sizeof(float), cudaMemcpyHostToDevice));
    }

    // Launch one kernel per stream
    for (int s = 0; s < NSTREAMS; s++) {
        CUDA_CHECK(cudaMemsetAsync(d_out[s], 0, sizeof(float), streams[s]));
        compute_sum<<<(N + BLOCK - 1) / BLOCK, BLOCK, BLOCK * sizeof(float), streams[s]>>>(
            d_in[s], d_out[s], N, ITERS);
        CUDA_CHECK(cudaGetLastError());
    }

    // BUG: only stream[0] is synchronized; streams 1-3 results may not be ready
    CUDA_CHECK(cudaStreamSynchronize(streams[0]));

    // Copy back results from all streams
    for (int s = 0; s < NSTREAMS; s++) {
        CUDA_CHECK(cudaMemcpy(&h_result[s], d_out[s], sizeof(float), cudaMemcpyDeviceToHost));
    }

    // Each result should be a large positive number (> 0)
    for (int s = 0; s < NSTREAMS; s++) {
        CHECK(h_result[s] > 0.0f);
    }

    for (int s = 0; s < NSTREAMS; s++) {
        CUDA_CHECK(cudaStreamDestroy(streams[s]));
        CUDA_CHECK(cudaFree(d_in[s]));
        CUDA_CHECK(cudaFree(d_out[s]));
        CUDA_CHECK(cudaFreeHost(h_in[s]));
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
