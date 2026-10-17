//! Infer Morph instructions and update context from exact snippets.

pub(super) fn infer_instruction(
    old: Option<&str>,
    new: Option<&str>,
    instruction: Option<&str>,
) -> String {
    instruction.map(str::to_string).or_else(|| old.zip(new).map(|(old, new)| {
        format!("Replace the target snippet exactly once.\nOld snippet:\n{old}\n\nNew snippet:\n{new}")
    })).unwrap_or_else(|| "Apply the requested update precisely.".to_string())
}

pub(super) fn infer_update(old: Option<&str>, new: Option<&str>, update: Option<&str>) -> String {
    update.map(str::to_string).or_else(|| old.zip(new).map(|(old, new)| {
        format!("// Replace this snippet:\n{old}\n// With this snippet:\n{new}\n// ...existing code...")
    })).unwrap_or_else(|| "// ...existing code...".to_string())
}
