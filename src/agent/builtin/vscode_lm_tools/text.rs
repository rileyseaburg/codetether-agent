//! Text compaction helpers for prompt snippets.

const MAX_DESCRIPTION_CHARS: usize = 180;

pub(super) fn compact(value: &str) -> String {
    let flat = value.replace('\n', " ");
    if flat.chars().count() <= MAX_DESCRIPTION_CHARS {
        flat
    } else {
        format!(
            "{}...",
            flat.chars()
                .take(MAX_DESCRIPTION_CHARS.saturating_sub(3))
                .collect::<String>()
        )
    }
}
