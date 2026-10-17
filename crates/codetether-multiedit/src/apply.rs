//! Choose the requested Morph or exact replacement path.
use super::{exact::apply_exact_replace, op::EditOp};
use codetether_morph_backend::Editor;
pub(super) async fn apply(
    content: &str,
    op: &EditOp,
    i: usize,
    editor: &dyn Editor,
) -> Result<String, String> {
    if op.use_morph {
        super::morph::apply(content, op, i, editor).await
    } else {
        let old_string = op.old_string.as_deref().unwrap_or_default();
        let new_string = op.new_string.as_deref().unwrap_or_default();
        apply_exact_replace(content, old_string, new_string, &op.file, i)
    }
}
