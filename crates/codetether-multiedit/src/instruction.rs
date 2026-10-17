//! Infer the Morph instruction from explicit fields or exact snippets.
use super::op::EditOp;
pub(super) fn infer(op: &EditOp) -> String {
    op
                    .instruction
                    .clone()
                    .or_else(|| {
                        op.old_string
                            .as_deref()
                            .zip(op.new_string.as_deref())
                            .map(|(old, new)| {
                                format!(
                                    "Replace the target snippet exactly once while preserving behavior.\nOld snippet:\n{old}\n\nNew snippet:\n{new}"
                                )
                            })
                    })
                    .unwrap_or_else(|| {
                        "Apply the requested update precisely and return only the updated file."
                            .to_string()
                    })
}
