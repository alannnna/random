// EXERCISE 1: Fix the wrong output index in the matrix transpose kernel.
//
// transpose should copy in[row][col] to out[col][row]. The output index
// is currently computed as `col * N + row` which is correct, but the
// source reads `in[col * N + row]` instead of `in[row * N + col]` —
// so the kernel copies the already-transposed position, effectively
// reading from the wrong location and producing an incorrect result.
//
// Fix: change the source read from `in[col * N + row]` to `in[row * N + col]`.
//
// Build & run:
//   cmake --build build --target ex01_transpose && ./build/module_04_warp_optimization/ex01_transpose

#include <cstdio>
#include <cmath>
#include <cuda_runtime.h>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)
#define CUDA_CHECK(call) do { cudaError_t _e=(call); if(_e!=cudaSuccess){ \
    fprintf(stderr,"CUDA error %s at line %d\n",cudaGetErrorString(_e),__LINE__); exit(1);} } while(0)

static const int N = 32;  // N×N matrix

__global__ void transpose(const float* in, float* out) {
    int col = blockIdx.x * blockDim.x + threadIdx.x;
    int row = blockIdx.y * blockDim.y + threadIdx.y;
    if (row >= N || col >= N) return;
    // BUG: reads from in[col*N+row] (already-transposed position) instead of in[row*N+col]
    out[col * N + row] = in[col * N + row];
}

int main() {
    float h_in[N * N], h_out[N * N];
    // Fill with unique values: in[r][c] = r*N + c
    for (int r = 0; r < N; r++)
        for (int c = 0; c < N; c++)
            h_in[r * N + c] = (float)(r * N + c);

    float *d_in, *d_out;
    CUDA_CHECK(cudaMalloc(&d_in,  N * N * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_out, N * N * sizeof(float)));
    CUDA_CHECK(cudaMemcpy(d_in, h_in, N * N * sizeof(float), cudaMemcpyHostToDevice));

    dim3 block(16, 16);
    dim3 grid((N + 15) / 16, (N + 15) / 16);
    transpose<<<grid, block>>>(d_in, d_out);
    CUDA_CHECK(cudaGetLastError());
    CUDA_CHECK(cudaDeviceSynchronize());

    CUDA_CHECK(cudaMemcpy(h_out, d_out, N * N * sizeof(float), cudaMemcpyDeviceToHost));

    // out[c][r] should equal in[r][c] = r*N + c
    bool all_ok = true;
    for (int r = 0; r < N && all_ok; r++)
        for (int c = 0; c < N && all_ok; c++)
            if (fabsf(h_out[c * N + r] - h_in[r * N + c]) > 0.5f) all_ok = false;
    CHECK(all_ok);

    // Spot-check: out[1][0] should equal in[0][1] = 1
    CHECK(fabsf(h_out[1 * N + 0] - h_in[0 * N + 1]) < 0.5f);

    cudaFree(d_in); cudaFree(d_out);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass + _fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
