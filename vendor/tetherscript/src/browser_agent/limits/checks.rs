//! Resource limit checks for browser page guards.

pub(crate) fn dom_bytes(html: &str) -> usize {
    html.len()
}

pub(crate) fn enforce_dom_bytes(
    operation: &str,
    html: &str,
    max_dom_bytes: usize,
) -> Result<(), String> {
    let current = dom_bytes(html);
    if current <= max_dom_bytes {
        return Ok(());
    }
    Err(format!(
        "{operation} blocked: DOM uses {current} bytes, exceeding max DOM bytes {max_dom_bytes}"
    ))
}
