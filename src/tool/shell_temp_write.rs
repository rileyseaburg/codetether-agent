//! Detection of shell commands that write into a system temp directory.

use crate::tool::temp_write_guard::denied_roots;

/// Shell utilities that mutate the filesystem.
const WRITE_COMMANDS: &[&str] = &[
    "chmod", "chown", "cp", "dd", "install", "ln", "mkdir", "mktemp", "mv", "rm", "rmdir", "tee",
    "touch", "truncate", "unzip", "tar",
];

/// Returns the temp path a command writes to, when one is present.
///
/// Detects both redirections (`echo x > /tmp/f`) and write-intent commands
/// (`mkdir /tmp/d`). Read-only usage such as `cat /tmp/f` is allowed so
/// inspecting existing temp state still works.
pub(super) fn detected(command: &str) -> Option<String> {
    let lower = command.to_ascii_lowercase();
    redirect_target(&lower).or_else(|| write_command_target(&lower))
}

fn redirect_target(command: &str) -> Option<String> {
    command
        .split('>')
        .skip(1)
        .flat_map(|tail| tail.split([';', '&', '|', '\n']).next())
        .flat_map(str::split_whitespace)
        .find_map(temp_word)
}

fn write_command_target(command: &str) -> Option<String> {
    command
        .split([';', '|', '&', '\n', '\r'])
        .find_map(segment_target)
}

fn segment_target(segment: &str) -> Option<String> {
    let words: Vec<&str> = segment.split_whitespace().collect();
    let writes = words
        .iter()
        .any(|word| WRITE_COMMANDS.contains(&executable(word)));
    writes.then(|| words.iter().find_map(|word| temp_word(word)))?
}

fn executable(word: &str) -> &str {
    word.trim_matches(['\'', '"', '(', ')'])
        .rsplit('/')
        .next()
        .unwrap_or("")
}

fn temp_word(word: &str) -> Option<String> {
    let cleaned = word.trim_matches(['\'', '"', '(', ')', ',']);
    let path = std::path::Path::new(cleaned);
    denied_roots()
        .iter()
        .any(|root| path.starts_with(root))
        .then(|| cleaned.to_string())
}

#[cfg(test)]
#[path = "shell_temp_write_tests.rs"]
mod tests;