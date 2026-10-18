//! Environment opt-in and model-path precedence for local CUDA.
use crate::Registry;

pub(super) fn register_local_cuda<H: crate::Host>(registry: &mut impl Registry) {
    if registry.contains("local_cuda") {
        return;
    }
    let enabled = std::env::var("CODETETHER_LOCAL_CUDA")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let name = std::env::var("LOCAL_CUDA_MODEL")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_MODEL"))
        .unwrap_or_else(|_| "qwen3.5-9b".into());
    let path = std::env::var("LOCAL_CUDA_MODEL_PATH")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_MODEL_PATH"))
        .ok();
    let tok = std::env::var("LOCAL_CUDA_TOKENIZER_PATH")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_TOKENIZER_PATH"))
        .ok();
    let arch = std::env::var("LOCAL_CUDA_ARCH")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_ARCH"))
        .ok();
    let has_config = std::env::var("LOCAL_CUDA_MODEL").is_ok()
        || std::env::var("CODETETHER_LOCAL_CUDA_MODEL").is_ok()
        || path.is_some()
        || tok.is_some()
        || arch.is_some();
    if !enabled && !has_config {
        return;
    }
    let result = if let Some(p) = path {
        H::local_cuda_with_paths(name.clone(), p, tok, arch)
    } else {
        H::local_cuda_new(name.clone())
    };
    match result {
        Ok(p) => {
            tracing::info!(model = %name, "Registered local_cuda from environment");
            registry.register(p);
        }
        Err(e) => tracing::warn!("Failed to init local_cuda from env: {e}"),
    }
}
