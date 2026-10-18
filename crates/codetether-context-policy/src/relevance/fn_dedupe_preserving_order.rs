//! dedupe preserving order policy.

pub(super) fn dedupe_preserving_order(items: &mut Vec<String>) {
    let mut seen: std::collections::HashSet<String> =
        std::collections::HashSet::with_capacity(items.len());
    items.retain(|item| seen.insert(item.clone()));
}
