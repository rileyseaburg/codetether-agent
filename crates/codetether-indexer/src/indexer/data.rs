//! data responsibilities for codebase indexing.

use super::*;
mod codebase_index;
pub use codebase_index::CodebaseIndex;
mod index_stats;
pub use index_stats::IndexStats;
mod indexed_file;
pub use indexed_file::IndexedFile;
mod knowledge_graph_stats;
pub use knowledge_graph_stats::KnowledgeGraphStats;
mod knowledge_graph;
pub use knowledge_graph::KnowledgeGraph;
mod knowledge_node;
pub use knowledge_node::KnowledgeNode;
mod knowledge_edge;
pub use knowledge_edge::KnowledgeEdge;
mod build_options;
pub use build_options::BuildOptions;
mod analyzed_file_knowledge;
pub(super) use analyzed_file_knowledge::AnalyzedFileKnowledge;
