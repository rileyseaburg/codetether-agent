//! Validate every operation while accumulating changes for repeated paths.
use super::{op::EditOp, validated::Validated};
use codetether_morph_backend::Editor;
use std::path::PathBuf;
pub(super) async fn validate(ops: &[EditOp], editor: &dyn Editor) -> Validated {
    let mut state = Validated {
        edits: Vec::with_capacity(ops.len()),
        ..Default::default()
    };
    for (i, op) in ops.iter().enumerate() {
        let path = PathBuf::from(&op.file);
        let content = match super::load::load(&path, op, i, &state.cache).await {
            Ok(c) => c,
            Err(e) => {
                state.errors.push(e);
                continue;
            }
        };
        let new_content = match super::apply::apply(&content, op, i, editor).await {
            Ok(c) => c,
            Err(e) => {
                state.errors.push(e);
                continue;
            }
        };
        state.cache.insert(path.clone(), new_content.clone());
        state.edits.push((path, content, new_content));
    }
    state
}
