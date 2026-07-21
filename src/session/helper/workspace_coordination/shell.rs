//! Conservative recognition of shell commands proven read-only.

#[path = "shell/git.rs"]
mod git;
#[path = "shell/query.rs"]
mod query;
#[path = "shell/systemctl.rs"]
mod systemctl;

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
        Some("git") => git::read(words, command),
        Some("systemctl") => systemctl::read(words),
        Some("find") => find_read(command),
        Some(_) if query::help_or_version(command) => true,
        Some(program) => SIMPLE.contains(&program),
        None => false,
    }
}

fn find_read(command: &str) -> bool {
    ![
        "-delete", "-exec", "-execdir", "-fprint", "-fprintf", "-fls", "-ok",
    ]
    .into_iter()
    .any(|flag| command.contains(flag))
}

#[cfg(test)]
#[path = "shell/tests.rs"]
mod tests;
