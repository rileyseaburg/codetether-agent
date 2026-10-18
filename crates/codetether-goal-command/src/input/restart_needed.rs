//! Identify human goal commands that require refreshing the session runtime.

pub(super) fn for_verb(verb: &str) -> bool {
    matches!(
        verb,
        "set" | "edit" | "override" | "budget" | "pause" | "resume" | "execute" | "done" | "clear"
    )
}
