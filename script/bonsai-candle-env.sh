# Source this only with a CI-built CodeTether binary containing native Bonsai CUDA support.
# No server, Python interpreter, or external inference process is used.
_bonsai_root="$HOME/.local/share/codetether/bonsai2/models"
if [ ! -f "$_bonsai_root/Ternary-Bonsai-2-27B-PQ2_0.gguf" ] || [ ! -f "$_bonsai_root/tokenizer.json" ]; then
    printf '%s\n' 'Bonsai PQ2 weights and matching tokenizer are required; see docs/native_bonsai.md.' >&2
    unset _bonsai_root
    return 1
fi
export LOCAL_CUDA_MODEL=ternary-bonsai-2-27b-pq2
export LOCAL_CUDA_MODEL_PATH="$_bonsai_root/Ternary-Bonsai-2-27B-PQ2_0.gguf"
export LOCAL_CUDA_TOKENIZER_PATH="$_bonsai_root/tokenizer.json"
export LOCAL_CUDA_ARCH=qwen35
export LOCAL_CUDA_DEVICE=cuda
export LOCAL_CUDA_MAX_TOKENS=512
export LOCAL_CUDA_TIMEOUT_MS=600000
unset _bonsai_root
printf '%s\n' 'Native model selector: local_cuda/ternary-bonsai-2-27b-pq2'
printf '%s\n' 'This does not change your default model. Verify the CUDA build and parity gates before selecting it.'
