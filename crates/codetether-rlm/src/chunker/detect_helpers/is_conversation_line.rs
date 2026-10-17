//! Classify conversation line.

pub(crate) fn is_conversation_line(line: &str) -> bool {
    let p = [
        "[User]:",
        "[Assistant]:",
        "[Human]:",
        "[AI]:",
        "User:",
        "Assistant:",
        "Human:",
        "AI:",
        "[Tool ",
        "<user>",
        "<assistant>",
        "<system>",
    ];
    p.iter().any(|p| line.starts_with(p))
}
