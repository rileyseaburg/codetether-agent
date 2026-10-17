//! Codebase indexing with separate scanning, graph, embedding, and CLI modules.

//! Codebase indexer
//!
//! Builds a lightweight, persistent JSON index of source files for fast
//! workspace introspection and downstream search/ranking workflows.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use walkdir::{DirEntry, WalkDir};
const INDEX_VERSION: u32 = 3;
const KNOWLEDGE_GRAPH_VERSION: u32 = 1;
const LOCAL_EMBEDDING_PROVIDER: &str = "local/hash-embedding";
const DEFAULT_EMBEDDING_PROVIDER: &str = "local";
const DEFAULT_EMBEDDING_MODEL: &str = "hash-v1";
const DISABLED_EMBEDDING_PROVIDER: &str = "disabled";
const DISABLED_EMBEDDING_MODEL: &str = "disabled";
const DEFAULT_EMBEDDING_DIMENSIONS: usize = 384;
const DEFAULT_EMBEDDING_BATCH_SIZE: usize = 32;
const DEFAULT_EMBEDDING_INPUT_CHARS: usize = 8_000;
const DEFAULT_EMBEDDING_MAX_RETRIES: u32 = 3;
const DEFAULT_EMBEDDING_RETRY_INITIAL_MS: u64 = 250;
const DEFAULT_EMBEDDING_RETRY_MAX_MS: u64 = 2_000;
const DEFAULT_RUN_KNOWLEDGE_MAX_FILE_SIZE_BYTES: u64 = 512 * 1024;

mod build;
mod data;
mod embedding;
mod graph;
mod parse;
mod persistence;
mod scan;
#[cfg(test)]
mod tests;

pub use build::{build_index, build_index_with_resolver};
use data::AnalyzedFileKnowledge;
pub use data::*;
use embedding::*;
use graph::*;
use parse::*;
pub use persistence::default_index_path;
pub use persistence::refresh_workspace_knowledge_snapshot;
use scan::*;

use crate::EmbeddingResolver;
