//! External stylesheet application for page resources.

use super::discover::ResourceReference;
use super::{url, ResourceKind, ResourceRegistry};

pub(crate) fn append_stylesheets(
    mut css: String,
    refs: &[ResourceReference],
    registry: &ResourceRegistry,
    base_url: &str,
) -> Result<String, String> {
    for reference in refs
        .iter()
        .filter(|item| item.kind == ResourceKind::Stylesheet)
    {
        let source = stylesheet(registry, base_url, &reference.url)
            .ok_or_else(|| missing(base_url, &reference.url))?;
        if !css.contains(source) {
            if !css.trim().is_empty() {
                css.push('\n');
            }
            css.push_str(source);
        }
    }
    Ok(css)
}

fn stylesheet<'a>(
    registry: &'a ResourceRegistry,
    base_url: &str,
    reference: &str,
) -> Option<&'a str> {
    url::candidates(base_url, reference)
        .into_iter()
        .find_map(|url| registry.text(ResourceKind::Stylesheet, &url))
}

fn missing(base_url: &str, reference: &str) -> String {
    let resolved = url::resolve(base_url, reference);
    format!(
        "missing external stylesheet resource: {} (resolved {})",
        reference, resolved
    )
}
