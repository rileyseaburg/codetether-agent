//! Safe read-only command prefixes.

const READ_ONLY: &[&str] = &[
    "pwd",
    "ls",
    "cat",
    "head",
    "tail",
    "grep",
    "rg",
    "find",
    "wc",
    "stat",
    "date",
    "true",
    "false",
    "whoami",
    "id",
    "uname",
    "printenv",
    "git rev-parse",
    "git remote get-url",
];

pub(super) fn read_only(command: &str) -> bool {
    let command = command.trim_start();
    READ_ONLY
        .iter()
        .any(|prefix| command == *prefix || command.starts_with(&format!("{prefix} ")))
}
