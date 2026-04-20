// EXERCISE 2: Replace the non-atomic increment with atomicAdd.
//
// histogram bins 4096 values into 16 buckets. Many threads map to the
// same bucket at the same time. Without an atomic operation, concurrent
// read-modify-write cycles overwrite each other's updates, producing a
// total count far less than N.
//
// Fix: replace `hist[bin]++` with `atomicAdd(&hist[bin], 1)`.
//
// Build & run:
//   cmake --build build --target ex02_histogram && ./build/module_02_parallel_patterns/ex02_histogram

#include <cstdio>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

static const int NUM_BINS = 16;

__global__ void histogram(const int* data, int* hist, int n) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= n) return;
    int bin = data[idx] % NUM_BINS;
    // BUG: non-atomic — concurrent threads on the same bin lose each other's writes
    hist[bin]++;
}

int main() {
    const int N = 4096;  // exactly 256 values per bin

    int h_data[N], h_hist[NUM_BINS];
    for (int i = 0; i < N; i++) h_data[i] = i;

    int *d_data, *d_hist;
    CUDA_CHECK(cudaMalloc(&d_data, N       * sizeof(int)));
    CUDA_CHECK(cudaMalloc(&d_hist, NUM_BINS * sizeof(int)));
    CUDA_CHECK(cudaMemcpy(d_data, h_data, N * sizeof(int), cudaMemcpyHostToDevice));

    for (int trial = 0; trial < 5; trial++) {
        CUDA_CHECK(cudaMemset(d_hist, 0, NUM_BINS * sizeof(int)));
        histogram<<<16, 256>>>(d_data, d_hist, N);
        CUDA_CHECK(cudaGetLastError());
        CUDA_CHECK(cudaDeviceSynchronize());

        CUDA_CHECK(cudaMemcpy(h_hist, d_hist, NUM_BINS * sizeof(int), cudaMemcpyDeviceToHost));

        int total = 0;
        bool bins_ok = true;
        for (int b = 0; b < NUM_BINS; b++) {
            total += h_hist[b];
            if (h_hist[b] != N / NUM_BINS) bins_ok = false;
        }
        CHECK(total == N);
        CHECK(bins_ok);
    }

    cudaFree(d_data); cudaFree(d_hist);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
