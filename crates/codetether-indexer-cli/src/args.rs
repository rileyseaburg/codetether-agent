//! Command-line schema for codebase indexing.
use clap::Parser;
use std::path::PathBuf;

/// File selection, embedding and output options for codebase indexing.
///
/// # Examples
/// ```
/// use clap::Parser;
/// let args = codetether_indexer_cli::IndexArgs::parse_from(["index", "--json"]);
/// assert!(args.json);
/// assert_eq!(args.embedding_provider, "local");
/// ```
#[derive(Parser, Debug)]
pub struct IndexArgs {
    /// Root directory to index (defaults to current directory)
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Output file for index JSON (defaults to CODETETHER_DATA_DIR/indexes/...)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Maximum file size to index in KiB
    #[arg(long, default_value = "1024")]
    pub max_file_size_kib: u64,

    /// Local open-source embedding model identifier
    #[arg(long, default_value = "hash-v1")]
    pub embedding_model: String,

    /// Embedding provider ID (`local`, `huggingface`, etc.)
    #[arg(long, default_value = "local")]
    pub embedding_provider: String,

    /// Embedding vector dimensions
    #[arg(long, default_value = "384")]
    pub embedding_dimensions: usize,

    /// Number of documents per embedding batch
    #[arg(long, default_value = "32")]
    pub embedding_batch_size: usize,

    /// Maximum retry attempts for a failed remote embedding batch
    #[arg(long, default_value = "3")]
    pub embedding_max_retries: u32,

    /// Initial retry backoff in milliseconds for remote embedding failures
    #[arg(long, default_value = "250")]
    pub embedding_retry_initial_ms: u64,

    /// Maximum retry backoff in milliseconds for remote embedding failures
    #[arg(long, default_value = "2000")]
    pub embedding_retry_max_ms: u64,

    /// Max characters per file used for embedding input
    #[arg(long, default_value = "8000")]
    pub embedding_input_chars: usize,

    /// Include hidden files/directories (dotfiles)
    #[arg(long, default_value_t = false)]
    pub include_hidden: bool,

    /// Output result as JSON
    #[arg(long)]
    pub json: bool,
}
