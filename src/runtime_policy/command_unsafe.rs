//! Shell forms that cannot use the read-only fast path.

const SHELL_SYNTAX: &[&str] = &[
    "&&", "||", ";", "|", "`", "$(", "\n", "\r", ">", "<", "&", "(", ")",
];

const FIND_WRITE_FLAGS: &[&str] = &[
    "-delete", "-exec", "-execdir", "-ok", "-okdir", "-fprint", "-fprintf", "-fls",
];

pub(super) fn rejected(command: &str) -> bool {
    has_shell_syntax(command)
        || unsafe_date(command)
        || unsafe_find(command)
        || unsafe_git(command)
        || unsafe_rg(command)
}

fn has_shell_syntax(command: &str) -> bool {
    SHELL_SYNTAX.iter().any(|syntax| command.contains(syntax))
}

fn unsafe_date(command: &str) -> bool {
    let words = words(command);
    words.first() == Some(&"date")
        && words[1..]
            .iter()
            .any(|word| matches!(*word, "-s" | "--set") || word.starts_with("--set="))
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

fn unsafe_rg(command: &str) -> bool {
    let words = words(command);
    words.first() == Some(&"rg")
        && words.iter().any(|word| {
            matches!(*word, "--pre" | "--hostname-bin")
                || word.starts_with("--pre=")
                || word.starts_with("--hostname-bin=")
        })
}

fn words(command: &str) -> Vec<&str> {
    command.split_whitespace().collect()
}
