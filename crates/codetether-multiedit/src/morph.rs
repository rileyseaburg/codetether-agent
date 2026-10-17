//! Apply Morph and fall back to exact snippets only when supplied.
use super::{exact::apply_exact_replace, op::EditOp};
use codetether_morph_backend::Editor;
pub(super) async fn apply(
    content: &str,
    op: &EditOp,
    i: usize,
    editor: &dyn Editor,
) -> Result<String, String> {
    let inferred_instruction = super::instruction::infer(op);
    let inferred_update = super::update::infer(op);
    match editor
        .apply(&content, &inferred_instruction, &inferred_update)
        .await
    {
        Ok(c) => Ok(c),
        Err(e) => {
            if let (Some(old_string), Some(new_string)) =
                (op.old_string.as_deref(), op.new_string.as_deref())
            {
                tracing::warn!(target:"codetether_agent::tool::multiedit",
                    file = %op.file,
                    error = %e,
                    "Morph backend failed for multiedit op; falling back to exact replacement"
                );
                match apply_exact_replace(&content, old_string, new_string, &op.file, i) {
                    Ok(c) => Ok(c),
                    Err(msg) => {
                        return Err(msg);
                    }
                }
            } else {
                return Err(format!("edits[{i}] {}: Morph backend failed: {e}", op.file));
            }
        }
    }
}
