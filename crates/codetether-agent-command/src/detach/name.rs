//! Unique background-agent name selection for `/detach`.

use super::Host;

/// Choose a unique agent name from `rest`, defaulting to `detached-N`.
pub(super) fn pick_name(app: &impl Host, rest: &str) -> String {
    let base = rest.trim();
    if !base.is_empty() && !app.contains_name(base) {
        return base.to_string();
    }
    (1..)
        .map(|n| format!("detached-{n}"))
        .find(|c| !app.contains_name(c))
        .unwrap_or_else(|| "detached".to_string())
}
