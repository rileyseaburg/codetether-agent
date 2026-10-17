//! Shared graph fixture.
use codetether_knowledge_graph::graph::*;
pub fn node(id: &str, file: &str) -> CodeNode {
    CodeNode {
        id: id.into(),
        name: id.into(),
        kind: CodeNodeKind::Function,
        file_path: file.into(),
        line: 1,
        embedding: None,
        last_modified: chrono::DateTime::UNIX_EPOCH,
    }
}
