//! Swarm truncate presentation.

pub(super) fn truncate_str(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.replace('\n', " ")
    } else {
        let mut end = max;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}...", s[..end].replace('\n', " "))
    }
}
