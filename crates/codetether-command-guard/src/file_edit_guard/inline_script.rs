//! Detection of inline interpreter scripts fed via stdin or `-c`/`-e`.
//!
//! `python3 - <<'EOF'`, `python -c "..."`, `perl -e`, `node -e`, and friends
//! run code the reviewer never sees as a file. Whether or not that code
//! writes, its effect is opaque, so the bash tool refuses it and points at
//! the structured edit tools. Running a script *file* stays allowed because
//! the file itself is reviewable.

const INTERPRETERS: &[&str] = &[
    "python", "python3", "perl", "ruby", "node", "deno", "bun", "php",
];

/// Returns a rejection reason when `lower` (already lowercased) invokes an
/// interpreter with an inline program instead of a script file.
pub(super) fn inline_script_reason(lower: &str) -> Option<&'static str> {
    let words: Vec<&str> = lower.split_whitespace().collect();
    for (index, word) in words.iter().enumerate() {
        let name = word.rsplit('/').next().unwrap_or(word);
        if !INTERPRETERS.iter().any(|item| name.starts_with(item)) {
            continue;
        }
        // Inline programs arrive as `-`, `<<`, or a `-c`/`-e` flag anywhere in
        // the leading flag run (`perl -pi -e`, `python -u -c`).
        let inline = words[index + 1..]
            .iter()
            .take_while(|arg| arg.starts_with('-') || **arg == "<<")
            .any(|arg| {
                *arg == "-" || *arg == "<<" || arg.starts_with("-c") || arg.starts_with("-e")
            });
        if inline {
            return Some(
                "inline interpreter scripts (stdin heredoc, -c, -e) are blocked; \
                 use the edit or multiedit tool, or run a reviewable script file",
            );
        }
    }
    None
}
