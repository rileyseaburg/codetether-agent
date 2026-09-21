# Source this with a CodeTether binary containing the dedicated Bonsai CUDA provider.
# No server, Python interpreter, or external inference process is used.
_bonsai_root="$HOME/.local/share/codetether/bonsai2/models"
if [ ! -f "$_bonsai_root/Ternary-Bonsai-2-27B-PQ2_0.gguf" ] || [ ! -f "$_bonsai_root/tokenizer.json" ]; then
    printf '%s\n' 'Bonsai PQ2 weights and matching tokenizer are required; see docs/native_bonsai.md.' >&2
    unset _bonsai_root
    return 1
fi
export CODETETHER_BONSAI=1
export BONSAI_MODEL_PATH="$_bonsai_root/Ternary-Bonsai-2-27B-PQ2_0.gguf"
export BONSAI_TOKENIZER_PATH="$_bonsai_root/tokenizer.json"
unset _bonsai_root
printf '%s\n' 'Native model selector: bonsai/ternary-bonsai-2-27b-pq2'
printf '%s\n' 'This does not change your default model. Verify the CUDA build and parity gates before selecting it.'