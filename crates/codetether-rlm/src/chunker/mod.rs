//! Semantic chunking for large contexts.
//!
//! Splits content intelligently at natural boundaries and prioritizes
//! chunks for token budget selection.

mod boundaries;
mod chunk;
mod compress;
mod detect;
mod detect_helpers;
mod estimate;
mod hints;
mod reassemble;
mod select;
mod split;
#[cfg(test)]
mod tests;
mod types;

pub use types::{Chunk, ChunkOptions, ChunkType, ContentType};

/// Semantic chunker facade — delegates to focused submodules.
pub struct RlmChunker;

impl RlmChunker {
    pub fn detect_content_type(content: &str) -> ContentType {
        detect::detect_content_type(content)
    }
    pub fn get_processing_hints(ct: ContentType) -> &'static str {
        hints::get_processing_hints(ct)
    }
    pub fn estimate_tokens(text: &str) -> usize {
        estimate::estimate_tokens(text)
    }
    pub fn chunk(content: &str, options: Option<ChunkOptions>) -> Vec<Chunk> {
        chunk::chunk(content, options)
    }
    pub fn select_chunks(chunks: &[Chunk], max_tokens: usize) -> Vec<Chunk> {
        select::select_chunks(chunks, max_tokens)
    }
    pub fn reassemble(chunks: &[Chunk]) -> String {
        reassemble::reassemble(chunks)
    }
    pub fn compress(content: &str, max_tokens: usize, options: Option<ChunkOptions>) -> String {
        compress::compress(content, max_tokens, options)
    }
}
