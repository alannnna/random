// EXERCISE 4: Replace the non-atomic increment with atomicAdd.
//
// count_above counts how many elements in an array exceed a threshold.
// 1024 threads concurrently read, increment, and write *counter, creating
// a classic read-modify-write race. Most updates are lost and the final
// count is almost always wrong.
//
// Fix: replace `(*counter)++` with `atomicAdd(counter, 1)`.
//
// Build & run:
//   cmake --build build --target ex04_atomic_counter && ./build/module_01_memory/ex04_atomic_counter

#include <cstdio>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

__global__ void count_above(const int* data, int* counter, int n, int threshold) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= n) return;
    if (data[idx] > threshold) {
        // BUG: non-atomic — concurrent threads lose each other's increments
        (*counter)++;
    }
}

int main() {
    const int N         = 4096;
    const int THRESHOLD = N / 2;  // values 0..N-1; upper half exceeds threshold
    const int EXPECTED  = N - THRESHOLD - 1;  // N/2 - 1 elements above threshold

    int h_data[N];
    for (int i = 0; i < N; i++) h_data[i] = i;

    int *d_data, *d_counter;
    CUDA_CHECK(cudaMalloc(&d_data,    N * sizeof(int)));
    CUDA_CHECK(cudaMalloc(&d_counter, sizeof(int)));
    CUDA_CHECK(cudaMemcpy(d_data, h_data, N * sizeof(int), cudaMemcpyHostToDevice));

    for (int trial = 0; trial < 5; trial++) {
        CUDA_CHECK(cudaMemset(d_counter, 0, sizeof(int)));
        count_above<<<16, 256>>>(d_data, d_counter, N, THRESHOLD);
        CUDA_CHECK(cudaGetLastError());
        CUDA_CHECK(cudaDeviceSynchronize());

        int h_count;
        CUDA_CHECK(cudaMemcpy(&h_count, d_counter, sizeof(int), cudaMemcpyDeviceToHost));
        CHECK(h_count == EXPECTED);
    }

    cudaFree(d_data); cudaFree(d_counter);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
