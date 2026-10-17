//! Read-only Git commands with an explicit `-C` workspace.

pub(super) fn scoped_read(command: &str) -> bool {
    let mut words = command.split_whitespace().peekable();
    if words.next() != Some("git") {
        return false;
    }
    while words.peek() == Some(&"-C") {
        words.next();
        if words.next().is_none() {
            return false;
        }
    }
    let Some(operation) = words.next() else {
        return false;
    };
    matches!(
        operation,
        "status" | "diff" | "log" | "show" | "rev-parse" | "ls-files" | "grep"
    ) || operation == "remote" && words.next() == Some("get-url")
}
