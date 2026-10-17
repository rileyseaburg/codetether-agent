//! Parse ordered edit operations before accessing the filesystem.
use super::{EditOperation, ToolResult};
use serde_json::Value;
pub(super) fn parse(input: &Value) -> Result<Vec<EditOperation>, ToolResult> {
    let edits_val = super::array::parse(input)?;
    let mut edits = Vec::new();
    for (i, edit_val) in edits_val.iter().enumerate() {
        let file = super::file::parse(edit_val, i)?;
        let old_string = super::old_string::parse(edit_val, i, &file)?;
        let new_string = super::new_string::parse(edit_val, i, &file, &old_string)?;
        edits.push(EditOperation {
            file,
            old_string,
            new_string,
        });
    }
    Ok(edits)
}
