// Native packed PQ2_0 matvec/batched reference kernel; no llama linkage.
// Packing specification: PrismML llama.cpp d8f26eec7 (MIT), group size 128.
__device__ float bonsai_half(unsigned short h) {
    unsigned s = (h & 0x8000u) << 16, e = (h >> 10) & 31u, m = h & 1023u;
    if (!e) {
        if (!m) return __uint_as_float(s);
        unsigned shifts = 0;
        while (!(m & 1024u)) { m <<= 1; ++shifts; }
        return __uint_as_float(s | ((113u - shifts) << 23) | ((m & 1023u) << 13));
    }
    if (e == 31u) return __uint_as_float(s | 0x7f800000u | (m << 13));
    return __uint_as_float(s | ((e + 112u) << 23) | (m << 13));
}
extern "C" __global__ void bonsai_pq2(const float *input, const unsigned char *weights, float *output, unsigned columns, unsigned rows) {
    __shared__ float partial[256];
    unsigned flat = blockIdx.x, row = flat % rows, batch = flat / rows, lane = threadIdx.x;
    float sum = 0.0f;
    for (unsigned col = lane; col < columns; col += 256) {
        unsigned long long block = ((unsigned long long)row * columns + col) / 128;
        const unsigned char *p = weights + block * 34;
        unsigned q = (p[2 + (col % 128) / 4] >> ((col % 4) * 2)) & 3u;
        float d = bonsai_half((unsigned short)(p[0] | ((unsigned)p[1] << 8)));
        sum += input[(unsigned long long)batch * columns + col] * ((int)q - 1) * d;
    }
    partial[lane] = sum; __syncthreads();
    for (unsigned stride = 128; stride; stride >>= 1) {
        if (lane < stride) partial[lane] += partial[lane + stride];
        __syncthreads();
    }
    if (!lane) output[flat] = partial[0];
}
