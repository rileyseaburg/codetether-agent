//! build responsibilities for codebase indexing.

use super::*;
mod build_index;
pub use build_index::build_index_with_resolver;
mod local;
pub use local::build_index;

#[derive(Default)]
struct IndexInputs {
    stats: IndexStats,
    files: Vec<IndexedFile>,
    embedding_inputs: Vec<String>,
    knowledge_inputs: Vec<AnalyzedFileKnowledge>,
}
mod scan_workspace;
use scan_workspace::scan_workspace;
mod scan_entry;
use scan_entry::scan_entry;
mod read_input;
use read_input::read_input;
mod make_record;
use make_record::make_record;
mod record_file;
use record_file::record_file;
mod embed_batch;
use embed_batch::embed_batch;
mod apply_vectors;
use apply_vectors::apply_vectors;
mod populate_embeddings;
use populate_embeddings::populate_embeddings;
