//! Vertex GLM provider compatibility export.
//!
//! Construct [VertexGlmProvider] from service-account JSON and use it through
//! the shared provider trait. Request, transport, and streaming code lives in
//! the dedicated workspace crate.
pub use codetether_provider_vertex_glm::VertexGlmProvider;
