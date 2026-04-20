// EXERCISE 4: Fix the double-buffer selection in the pipeline loop.
//
// A double-buffering pattern uses two device buffers (buf[0] and buf[1])
// so that the GPU can process one chunk while the next chunk is being
// transferred. The current iteration's data should go into buf[iter % 2].
//
// Bug: the kernel always reads from buf[0] regardless of which buffer was
// just filled. The second chunk is copied into buf[1] but the kernel
// still processes the stale data in buf[0].
//
// Fix: pass buf[iter % 2] to the kernel instead of always buf[0].
//
// Build & run:
//   cmake --build build --target ex04_double_buffer && ./build/module_03_streams/ex04_double_buffer

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

__global__ void negate(const float* in, float* out, int n) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < n) out[idx] = -in[idx];
}

int main() {
    const int CHUNK = 512;
    const int NCHUNKS = 2;

    // Two host chunks with distinct values
    float h_chunk[NCHUNKS][CHUNK];
    for (int i = 0; i < CHUNK; i++) h_chunk[0][i] =  1.0f;  // chunk 0: all  1
    for (int i = 0; i < CHUNK; i++) h_chunk[1][i] =  3.0f;  // chunk 1: all  3

    float *h_out[NCHUNKS];
    for (int c = 0; c < NCHUNKS; c++)
        CUDA_CHECK(cudaMallocHost(&h_out[c], CHUNK * sizeof(float)));

    float *d_buf[2], *d_out[NCHUNKS];
    for (int b = 0; b < 2; b++)
        CUDA_CHECK(cudaMalloc(&d_buf[b], CHUNK * sizeof(float)));
    for (int c = 0; c < NCHUNKS; c++)
        CUDA_CHECK(cudaMalloc(&d_out[c], CHUNK * sizeof(float)));

    cudaStream_t stream;
    CUDA_CHECK(cudaStreamCreate(&stream));

    for (int iter = 0; iter < NCHUNKS; iter++) {
        // Fill buf[iter % 2] with this iteration's chunk
        CUDA_CHECK(cudaMemcpyAsync(d_buf[iter % 2], h_chunk[iter],
                                   CHUNK * sizeof(float), cudaMemcpyHostToDevice, stream));
        // BUG: always reads from d_buf[0] — should be d_buf[iter % 2]
        negate<<<(CHUNK + 255) / 256, 256, 0, stream>>>(d_buf[0], d_out[iter], CHUNK);
        CUDA_CHECK(cudaGetLastError());
        CUDA_CHECK(cudaMemcpyAsync(h_out[iter], d_out[iter],
                                   CHUNK * sizeof(float), cudaMemcpyDeviceToHost, stream));
    }

    CUDA_CHECK(cudaStreamSynchronize(stream));

    // chunk 0: negate(1.0) = -1.0
    CHECK(fabsf(h_out[0][0] - (-1.0f)) < 0.01f);
    // chunk 1: negate(3.0) = -3.0  (fails without fix — gets -1.0 from stale buf[0])
    CHECK(fabsf(h_out[1][0] - (-3.0f)) < 0.01f);

    CUDA_CHECK(cudaStreamDestroy(stream));
    for (int b = 0; b < 2; b++) CUDA_CHECK(cudaFree(d_buf[b]));
    for (int c = 0; c < NCHUNKS; c++) {
        CUDA_CHECK(cudaFree(d_out[c]));
        CUDA_CHECK(cudaFreeHost(h_out[c]));
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
