//! In-memory code graph and borrowed symbol lookups.
//!
//! [KnowledgeGraph] owns [CodeNode] values and [CodeEdge] relationships.
//! Use its lookup methods to find callers and symbols within a file.
mod lookup;
mod model;
mod storage;
pub use model::{CodeEdge, CodeNode, CodeNodeKind, EdgeKind};
pub use storage::KnowledgeGraph;
