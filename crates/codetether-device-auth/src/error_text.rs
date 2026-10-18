//! Device authorization error_text responsibilities.

pub(super) fn truncate_body(body: &str) -> String {
    const MAX_LEN: usize = 300;
    if body.len() <= MAX_LEN {
        body.to_string()
    } else {
        format!("{}...", &body[..MAX_LEN])
    }
}
