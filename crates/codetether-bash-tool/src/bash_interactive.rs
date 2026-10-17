//! Detection of commands that cannot safely run without a terminal.

#[cfg(test)]
#[path = "bash_interactive_tests.rs"]
mod tests;

const SHELLS: &[&str] = &["bash", "sh", "zsh", "ksh", "fish"];

pub(super) fn shell_reason(command: &str) -> Option<&'static str> {
    command
        .split([';', '\n', '|', '&'])
        .find_map(segment_shell_reason)
}

fn segment_shell_reason(segment: &str) -> Option<&'static str> {
    let mut tokens = segment.split_ascii_whitespace();
    let executable = tokens.find(|token| {
        !token.contains('=') && !matches!(*token, "env" | "command" | "exec" | "sudo")
    })?;
    let name = executable.trim_matches(['\'', '"']).rsplit('/').next()?;
    if !SHELLS.contains(&name) {
        return None;
    }
    tokens
        .any(is_interactive_flag)
        .then_some("Interactive shell flags require a TTY, but the bash tool is noninteractive.")
}

fn is_interactive_flag(arg: &str) -> bool {
    arg == "--interactive"
        || (arg.starts_with('-') && !arg.starts_with("--") && arg[1..].contains('i'))
}
