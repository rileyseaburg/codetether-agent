//! Parse, validate without writes, then write and report.
use super::ToolResult;
use anyhow::Result;
use codetether_morph_backend::Editor;
use serde_json::Value;
pub(super) async fn execute(params: Value, editor: &dyn Editor) -> Result<ToolResult> {
    let edits_arr = match super::input::parse(&params) {
        Ok(a) => a,
        Err(e) => return Ok(e),
    };
    let ops = match super::op::parse(edits_arr) {
        Ok(o) => o,
        Err(e) => return Ok(e),
    };
    let state = super::validate::validate(&ops, editor).await;
    if !state.errors.is_empty() {
        return Ok(super::report::validation(&state.errors, ops.len()));
    }
    let (written, write_errors) = super::write::write(&state.edits, &state.cache).await?;
    if !write_errors.is_empty() {
        return Ok(super::report::write_errors(&write_errors));
    }
    Ok(super::report::success(
        &state.edits,
        written.len(),
        ops.len(),
    ))
}
