//! UTF-8-safe previews for real-time swarm events.

#[cfg(test)]
#[path = "preview_tests.rs"]
mod tests;

pub(super) fn text(value: &str, maximum: usize) -> String {
    if value.len() <= maximum {
        return value.to_string();
    }
    let mut end = maximum;
    while end > 0 && !value.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &value[..end])
}
