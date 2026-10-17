//! Shell forms that cannot use the read-only fast path.

const SHELL_SYNTAX: &[&str] = &[
    "&&", "||", ";", "|", "`", "$(", "\n", "\r", ">", "<", "&", "(", ")",
];

const FIND_WRITE_FLAGS: &[&str] = &[
    "-delete", "-exec", "-execdir", "-ok", "-okdir", "-fprint", "-fprintf",
];

pub(super) fn rejected(command: &str) -> bool {
    has_shell_syntax(command) || unsafe_find(command) || unsafe_git(command)
}

fn has_shell_syntax(command: &str) -> bool {
    SHELL_SYNTAX.iter().any(|syntax| command.contains(syntax))
}

fn unsafe_find(command: &str) -> bool {
    let words = words(command);
    words.first() == Some(&"find")
        && words
            .iter()
            .any(|word| FIND_WRITE_FLAGS.iter().any(|flag| word.starts_with(flag)))
}

fn unsafe_git(command: &str) -> bool {
    let words = words(command);
    words.first() == Some(&"git")
        && words
            .iter()
            .any(|word| *word == "-o" || word.starts_with("--output"))
}

fn words(command: &str) -> Vec<&str> {
    command.split_whitespace().collect()
}
