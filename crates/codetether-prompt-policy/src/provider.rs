//! Recognize supported local CUDA provider aliases.

/// Return whether a provider name refers to a local CUDA-backed provider.
///
/// # Examples
///
/// ```rust
/// use codetether_prompt_policy::is_local_cuda_provider;
///
/// assert!(is_local_cuda_provider("local_cuda"));
/// assert!(is_local_cuda_provider("local-cuda"));
/// assert!(!is_local_cuda_provider("openai"));
/// ```
/// # Arguments
/// Receives the selected provider identifier.
/// # Returns
/// True for a recognized local CUDA provider alias.
pub fn is_local_cuda_provider(provider: &str) -> bool {
    matches!(provider, "local-cuda" | "local_cuda" | "localcuda")
}
