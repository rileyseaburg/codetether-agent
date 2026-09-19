// Normalized Sylvester Walsh-Hadamard blocks, with explicit per-channel signs.
extern "C" __global__ void bonsai_hadamard(const float *input, const float *signs, float *output, unsigned width, unsigned inverse) {
    __shared__ float data[1024];
    unsigned offset = blockIdx.x * 1024, feature = offset % width;
    for (unsigned i = threadIdx.x; i < 1024; i += 256) {
        float x = input[offset + i];
        data[i] = inverse ? x : x * signs[feature + i];
    }
    __syncthreads();
    for (unsigned stride = 1; stride < 1024; stride <<= 1) {
        float next[4];
        for (unsigned n = 0; n < 4; ++n) {
            unsigned i = threadIdx.x + n * 256;
            next[n] = (i & stride) ? data[i ^ stride] - data[i] : data[i] + data[i ^ stride];
        }
        __syncthreads();
        for (unsigned n = 0; n < 4; ++n) data[threadIdx.x + n * 256] = next[n];
        __syncthreads();
    }
    for (unsigned i = threadIdx.x; i < 1024; i += 256) {
        float x = data[i] * (1.0f / 32.0f);
        output[offset + i] = inverse ? x * signs[feature + i] : x;
    }
}
