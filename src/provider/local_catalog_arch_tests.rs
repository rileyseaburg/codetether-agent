//! Architecture-resolution tests for the local GGUF catalog.

use super::{NVIDIA_MODELS, arch, arch_or_env};

#[test]
fn every_arch_has_a_candle_loader() {
    // Mirrors the match arms in `cognition::thinker`.
    let supported = ["llama", "qwen2", "qwen3", "qwen3moe", "qwen3_moe"];
    for model in NVIDIA_MODELS {
        assert!(
            supported.contains(&model.arch),
            "{} declares arch '{}' with no candle loader",
            model.id,
            model.arch
        );
    }
}

#[test]
fn catalog_supplies_arch_independently_of_env() {
    assert_eq!(arch("nemotron-nano-8b-v1").as_deref(), Some("llama"));
    assert!(arch("totally-unknown-model").is_none());
}

#[test]
fn arch_or_env_prefers_explicit_override() {
    // Ambient env may set an override; when present it must win.
    match std::env::var("LOCAL_CUDA_ARCH") {
        Ok(forced) if !forced.trim().is_empty() => {
            assert_eq!(arch_or_env("nemotron-nano-8b-v1"), Some(forced));
        }
        _ => assert_eq!(arch_or_env("nemotron-nano-8b-v1").as_deref(), Some("llama")),
    }
}
