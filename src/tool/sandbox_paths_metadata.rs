use anyhow::{Result, anyhow};

const PROTECTED: &[&str] = &[".git", ".codetether", ".codex", ".agents"];
const WRITE_COMMANDS: &[&str] = &[
    "chmod", "chown", "cp", "mkdir", "mv", "rm", "tee", "touch", "truncate",
];

pub(super) fn deny_protected_writes(args: &[String]) -> Result<()> {
    for arg in args {
        let normalized = normalize(arg);
        if mentions_protected_segment(&normalized) && has_write_intent(&normalized) {
            return Err(anyhow!("Sandbox denied write to protected metadata path"));
        }
    }
    Ok(())
}

fn normalize(value: &str) -> String {
    value.to_ascii_lowercase().replace(['"', '\'', '\\'], "")
}

fn mentions_protected_segment(value: &str) -> bool {
    value
        .split(|ch: char| !(ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-')))
        .any(|part| PROTECTED.contains(&part))
}

fn has_write_intent(value: &str) -> bool {
    redirects_to_protected(value)
        || WRITE_COMMANDS
            .iter()
            .any(|command| contains_shell_word(value, command))
}

fn redirects_to_protected(value: &str) -> bool {
    value.split('>').skip(1).any(|tail| {
        let target = tail
            .split(|ch| matches!(ch, ';' | '&' | '|' | '\n'))
            .next()
            .unwrap_or("");
        mentions_protected_segment(target)
    })
}

fn contains_shell_word(haystack: &str, needle: &str) -> bool {
    haystack
        .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'))
        .any(|word| word == needle)
}

#[cfg(test)]
#[path = "sandbox_paths_metadata_tests.rs"]
mod tests;
