//! Catalog of NVIDIA GGUF models that candle can load in-process.
//!
//! Entries are restricted to architectures decoded by
//! [`crate::cognition`] and to `GgmlDType` variants candle accepts, so
//! `codetether models` never advertises a model that fails at load time.

/// A locally-inferenced GGUF model exposed through `local_cuda`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalModel {
    /// Selectable id for `codetether run --model`.
    pub id: &'static str,
    /// GGUF `general.architecture`, mapped to a candle loader.
    pub arch: &'static str,
    /// Native context window in tokens.
    pub context_window: usize,
}

/// NVIDIA models verified loadable by candle's quantized llama decoder.
pub const NVIDIA_MODELS: &[LocalModel] = &[LocalModel {
    id: "nemotron-nano-8b-v1",
    arch: "llama",
    context_window: 131_072,
}];

/// Look up a catalog entry by its selectable id.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::local_catalog;
///
/// let model = local_catalog::find("nemotron-nano-8b-v1").unwrap();
/// assert_eq!(model.arch, "llama");
/// assert!(local_catalog::find("nemotron-nano-12b-v2").is_none());
/// ```
pub fn find(id: &str) -> Option<&'static LocalModel> {
    NVIDIA_MODELS.iter().find(|model| model.id == id)
}

/// Candle architecture for `id`, or `None` when uncatalogued.
pub fn arch(id: &str) -> Option<String> {
    find(id).map(|model| model.arch.to_string())
}

/// Explicit `LOCAL_CUDA_ARCH` override, else the catalogued architecture.
///
/// Env vars win so operators can force an architecture for uncatalogued
/// GGUF files without a code change.
pub fn arch_or_env(id: &str) -> Option<String> {
    ["LOCAL_CUDA_ARCH", "CODETETHER_LOCAL_CUDA_ARCH"]
        .iter()
        .find_map(|key| std::env::var(key).ok().filter(|v| !v.trim().is_empty()))
        .or_else(|| arch(id))
}

/// Catalogued context window for `id`, falling back to a safe default.
pub fn context_window(id: &str) -> usize {
    find(id).map_or(8192, |model| model.context_window)
}

#[cfg(test)]
#[path = "local_catalog_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "local_catalog_arch_tests.rs"]
mod arch_tests;
