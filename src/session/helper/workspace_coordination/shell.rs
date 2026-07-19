//! Conservative recognition of shell commands proven read-only.

const SIMPLE: &[&str] = &[
    "cat", "cut", "df", "diff", "du", "echo", "grep", "head", "jq", "ls", "pwd", "readlink",
    "realpath", "rg", "stat", "tail", "test", "tr", "type", "uniq", "wc", "which",
];

pub(super) fn read_only(command: &str) -> bool {
    if command.trim().is_empty() || hazardous(command) {
        return false;
    }
    command
        .split(['|', '\n'])
        .all(|part| simple_read(part.trim()))
}

fn hazardous(command: &str) -> bool {
    [">", ";", "&", "||", "`", "$(", "${", "<(", ">("]
        .into_iter()
        .any(|marker| command.contains(marker))
}

fn simple_read(command: &str) -> bool {
    let mut words = command.split_whitespace();
    match words.next() {
        Some("git") => git_read(words.next(), command),
        Some("find") => find_read(command),
        Some(program) => SIMPLE.contains(&program),
        None => false,
    }
}

fn git_read(operation: Option<&str>, command: &str) -> bool {
    if command.contains("--output") {
        return false;
    }
    matches!(
        operation,
        Some("status" | "diff" | "log" | "show" | "rev-parse" | "ls-files" | "grep")
    )
}

fn find_read(command: &str) -> bool {
    ![
        "-delete", "-exec", "-execdir", "-fprint", "-fprintf", "-fls", "-ok",
    ]
    .into_iter()
    .any(|flag| command.contains(flag))
}
