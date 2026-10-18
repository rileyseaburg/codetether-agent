//! Read-only Git operation recognition, including global options.

pub(super) fn read(mut words: std::str::SplitWhitespace<'_>, command: &str) -> bool {
    if command.contains("--output") {
        return false;
    }
    while let Some(word) = words.next() {
        match word {
            "-C" | "-c" | "--git-dir" | "--work-tree" => {
                words.next();
            }
            option if option.starts_with('-') => {}
            operation => return operation_read(operation),
        }
    }
    false
}

fn operation_read(operation: &str) -> bool {
    matches!(
        operation,
        "status" | "diff" | "log" | "show" | "rev-parse" | "ls-files" | "grep" | "branch"
    )
}
