//! Parsed edit operations, independent of filesystem access.
use super::ToolResult;
use serde_json::Value;
pub(super) struct EditOp {
    pub file: String,
    pub old_string: Option<String>,
    pub new_string: Option<String>,
    pub instruction: Option<String>,
    pub update: Option<String>,
    pub use_morph: bool,
}

pub(super) fn parse(edits_arr: &[Value]) -> Result<Vec<EditOp>, ToolResult> {
    let mut ops = Vec::with_capacity(edits_arr.len());
    let morph_enabled = codetether_morph_backend::should_use_morph_backend();
    for (i, entry) in edits_arr.iter().enumerate() {
        let file = super::op_file::parse(entry, i)?;
        let op = super::op_fields::parse(entry, file, morph_enabled);
        super::op_validate::validate(&op, i)?;
        ops.push(op);
    }
    Ok(ops)
}
