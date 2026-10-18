//! Environment inputs and default model names used by RLM selection.

pub(super) fn local_cuda_runtime_configured() -> bool {
    let model_path = std::env::var("LOCAL_CUDA_MODEL_PATH")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_MODEL_PATH"))
        .ok();
    let tokenizer_path = std::env::var("LOCAL_CUDA_TOKENIZER_PATH")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_TOKENIZER_PATH"))
        .ok();
    model_path.is_some() && tokenizer_path.is_some()
}

pub(super) fn local_cuda_model_name() -> String {
    std::env::var("LOCAL_CUDA_MODEL")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_MODEL"))
        .unwrap_or_else(|_| "qwen3.5-9b".to_string())
}

pub(super) fn default_openrouter_rlm_model() -> String {
    std::env::var("CODETETHER_RLM_DEFAULT_MODEL")
        .or_else(|_| std::env::var("OPENROUTER_RLM_MODEL"))
        .unwrap_or_else(|_| "qwen/qwen3.5-35b-a3b".to_string())
}
