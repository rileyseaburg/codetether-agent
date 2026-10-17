//! Recognize common executable wrappers and their value-taking flags.

pub(super) fn shell(word: &str) -> bool {
    matches!(basename(word), "sh" | "bash" | "dash" | "zsh")
}

pub(super) fn takes_value(word: &str) -> bool {
    matches!(
        word,
        "-u" | "-g" | "--user" | "--group" | "--chdir" | "--signal"
    )
}

pub(super) fn allowed(word: &str) -> bool {
    matches!(
        basename(word),
        "env" | "command" | "exec" | "sudo" | "nohup" | "timeout" | "time"
    ) || word.starts_with('-')
        || word.contains('=')
        || word
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.' || c == 's')
}

fn basename(word: &str) -> &str {
    word.rsplit('/').next().unwrap_or("")
}
