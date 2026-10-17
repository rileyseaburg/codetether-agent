//! Infer the Morph update from explicit fields or exact snippets.
use super::op::EditOp;
pub(super) fn infer(op: &EditOp) -> String {
    op
                    .update
                    .clone()
                    .or_else(|| {
                        op.old_string
                            .as_deref()
                            .zip(op.new_string.as_deref())
                            .map(|(old, new)| {
                                format!(
                                    "// Replace this snippet:\n{old}\n// With this snippet:\n{new}\n// ...existing code..."
                                )
                            })
                    })
                    .unwrap_or_else(|| "// ...existing code...".to_string())
}
