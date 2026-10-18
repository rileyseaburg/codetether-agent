//! Read-only systemd unit inspection recognition.

pub(super) fn read(mut words: std::str::SplitWhitespace<'_>) -> bool {
    while let Some(word) = words.next() {
        if word.starts_with('-') {
            continue;
        }
        return matches!(
            word,
            "status"
                | "is-active"
                | "is-enabled"
                | "is-failed"
                | "show"
                | "cat"
                | "list-units"
                | "list-unit-files"
                | "list-dependencies"
                | "get-default"
        );
    }
    false
}
