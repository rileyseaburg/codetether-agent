//! Core types for semantic chunking.

use serde::{Deserialize, Serialize};

/// Content type for optimized processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Code,
    Documents,
    Logs,
    Conversation,
    Mixed,
}

/// A chunk of content with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub content: String,
    #[serde(rename = "type")]
    pub chunk_type: ChunkType,
    pub start_line: usize,
    pub end_line: usize,
    pub tokens: usize,
    /// Higher = more important to keep.
    pub priority: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChunkType {
    Code,
    Text,
    ToolOutput,
    Conversation,
}

/// Options for chunking.
#[derive(Debug, Clone)]
pub struct ChunkOptions {
    /// Maximum tokens per chunk.
    pub max_chunk_tokens: usize,
    /// Number of recent lines to always preserve.
    pub preserve_recent: usize,
}

impl Default for ChunkOptions {
    fn default() -> Self {
        Self {
            max_chunk_tokens: 4000,
            preserve_recent: 100,
        }
    }
}
