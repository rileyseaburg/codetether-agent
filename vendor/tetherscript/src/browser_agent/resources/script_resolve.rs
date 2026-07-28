//! Resolution helpers for deterministic script resources.

use super::{url, ResourceKind, ResourceRegistry};

pub(crate) fn text<'a>(
    registry: &'a ResourceRegistry,
    base_url: &str,
    reference: &str,
) -> Option<(String, &'a str)> {
    url::candidates(base_url, reference)
        .into_iter()
        .find_map(|url| {
            registry
                .text(ResourceKind::Script, &url)
                .map(|text| (url, text))
        })
}

pub(crate) fn missing(base_url: &str, reference: &str) -> String {
    let resolved = url::resolve(base_url, reference);
    format!(
        "missing external script resource: {} (resolved {})",
        reference, resolved
    )
}
