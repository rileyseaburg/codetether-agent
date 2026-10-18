//! Guarded tool names and identical-attempt threshold.

/// Edit-family tools whose repetitions we guard against.
pub(super) const EDIT_TOOLS: &[&str] = &[
    "edit",
    "write",
    "multiedit",
    "patch",
    "apply_patch",
    "confirm_edit",
];

/// Identical attempts allowed before the guard fires on the next one.
pub(super) const THRESHOLD: u8 = 3;
