// EXERCISE 3: Fix the in-place Hillis-Steele scan by using a double buffer.
//
// An inclusive prefix scan of [1,1,1,...,1] should produce [1,2,3,...,N].
// The Hillis-Steele algorithm does log2(N) passes. In pass d, each thread
// at position i reads sdata[i] and sdata[i-d] and writes the sum back to
// sdata[i], all in shared memory.
//
// Bug: reading from and writing to the same array in-place means that thread i
// may read from position i-d AFTER another thread has already updated it in
// this pass, producing wrong (over-accumulated) results.
//
// Fix: use two shared arrays (ping and pong) and alternate between them:
// read from src[], write to dst[], then swap src/dst each pass.
//
// Build & run:
//   cmake --build build --target ex03_prefix_scan && ./build/module_02_parallel_patterns/ex03_prefix_scan

#include <cstdio>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

static const int BLOCK = 64;  // must be a power of 2

// BUG: in-place scan — reads and writes the same shared array, causing RAW hazards
__global__ void scan_buggy(const float* in, float* out, int n) {
    extern __shared__ float sdata[];
    int tid = threadIdx.x;
    sdata[tid] = (tid < n) ? in[tid] : 0.0f;
    __syncthreads();

    for (int d = 1; d < blockDim.x; d <<= 1) {
        float val = sdata[tid];
        if (tid >= d) val += sdata[tid - d];  // reads from position that may already be updated
        __syncthreads();
        sdata[tid] = val;
        __syncthreads();
    }

    if (tid < n) out[tid] = sdata[tid];
}

int main() {
    const int N = BLOCK;

    float h_in[N], h_out[N];
    for (int i = 0; i < N; i++) h_in[i] = 1.0f;

    float *d_in, *d_out;
    CUDA_CHECK(cudaMalloc(&d_in,  N * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_out, N * sizeof(float)));
    CUDA_CHECK(cudaMemcpy(d_in, h_in, N * sizeof(float), cudaMemcpyHostToDevice));

    // Single block: shared memory holds two copies for the double-buffer fix
    scan_buggy<<<1, BLOCK, BLOCK * sizeof(float)>>>(d_in, d_out, N);
    CUDA_CHECK(cudaGetLastError());
    CUDA_CHECK(cudaDeviceSynchronize());

    CUDA_CHECK(cudaMemcpy(h_out, d_out, N * sizeof(float), cudaMemcpyDeviceToHost));

    // Inclusive scan of all-1s: out[i] should equal i+1
    bool all_ok = true;
    for (int i = 0; i < N; i++) {
        if (h_out[i] != (float)(i + 1)) { all_ok = false; break; }
    }
    CHECK(all_ok);
    CHECK(h_out[N - 1] == (float)N);

    cudaFree(d_in); cudaFree(d_out);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
