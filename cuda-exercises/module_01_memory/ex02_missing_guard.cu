// EXERCISE 2: Add a bounds check to prevent out-of-bounds writes.
//
// fill() writes 1.0f into every slot of d_a. N=1000, BLOCK=256 → 4 blocks
// → 1024 threads. Threads 1000..1023 compute idx >= N and, without a guard,
// write 1.0f into d_b[0..23] (d_a and d_b are allocated as a contiguous
// block, so d_a[1000] == d_b[0]).
//
// Fix: add `if (idx >= n) return;` at the top of fill().
//
// Build & run:
//   cmake --build build --target ex02_missing_guard && ./build/module_01_memory/ex02_missing_guard

#include <cstdio>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

static const int N     = 1000;
static const int BLOCK = 256;
static const int GRID  = (N + BLOCK - 1) / BLOCK;  // 4 blocks = 1024 threads

__global__ void fill(float* a, float val, int n) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    // BUG: no bounds check — threads idx >= n write past the end of a[]
    a[idx] = val;
}

int main() {
    // Allocate d_a and d_b as one contiguous region so OOB writes hit d_b
    float* workspace;
    CUDA_CHECK(cudaMalloc(&workspace, 2 * N * sizeof(float)));
    float* d_a = workspace;
    float* d_b = workspace + N;

    // d_b is pre-filled with a sentinel value
    const float SENTINEL = -999.0f;
    float h_sentinel[N];
    for (int i = 0; i < N; i++) h_sentinel[i] = SENTINEL;
    CUDA_CHECK(cudaMemcpy(d_b, h_sentinel, N * sizeof(float), cudaMemcpyHostToDevice));

    // Kernel fills d_a with 1.0f — threads 1000..1023 spill into d_b
    fill<<<GRID, BLOCK>>>(d_a, 1.0f, N);
    CUDA_CHECK(cudaGetLastError());
    CUDA_CHECK(cudaDeviceSynchronize());

    float h_b[N];
    CUDA_CHECK(cudaMemcpy(h_b, d_b, N * sizeof(float), cudaMemcpyDeviceToHost));

    // d_b should be untouched (all SENTINEL); OOB writes corrupt h_b[0..23]
    bool b_intact = true;
    for (int i = 0; i < N; i++) {
        if (h_b[i] != SENTINEL) { b_intact = false; break; }
    }
    CHECK(b_intact);

    // Also verify d_a was filled correctly
    float h_a[N];
    CUDA_CHECK(cudaMemcpy(h_a, d_a, N * sizeof(float), cudaMemcpyDeviceToHost));
    bool a_ok = true;
    for (int i = 0; i < N; i++) {
        if (h_a[i] != 1.0f) { a_ok = false; break; }
    }
    CHECK(a_ok);

    cudaFree(workspace);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
