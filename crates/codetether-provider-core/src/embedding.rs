use crate::Usage;

/// Request to generate embeddings.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::EmbeddingRequest;
/// let request = EmbeddingRequest {
///     model: "embedding-model".into(), inputs: vec!["hello".into()],
/// };
/// assert_eq!(request.inputs.len(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct EmbeddingRequest {
    /// Embedding model identifier.
    pub model: String,
    /// Text inputs to embed.
    pub inputs: Vec<String>,
}

/// Response from an embedding request.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::{EmbeddingResponse, Usage};
/// let response = EmbeddingResponse {
///     embeddings: vec![vec![0.1, 0.2]], usage: Usage::default(),
/// };
/// assert_eq!(response.embeddings.len(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct EmbeddingResponse {
    /// One float vector per input.
    pub embeddings: Vec<Vec<f32>>,
    /// Token usage.
    pub usage: Usage,
}
