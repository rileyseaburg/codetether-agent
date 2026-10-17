//! Serializable nodes, edges, and their classifications.
//!
//! Construct [CodeNode] values and connect their identifiers with [CodeEdge].
//! [CodeNodeKind] describes symbols; [EdgeKind] describes relationships.
mod edge;
mod node;
pub use edge::{CodeEdge, EdgeKind};
pub use node::{CodeNode, CodeNodeKind};
