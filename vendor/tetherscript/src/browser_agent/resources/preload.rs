//! DOM scanning for passive preload resource references.

use crate::browser::{Document, Element};

use super::{discover::ResourceReference, walk, ResourceKind};

pub(crate) fn collect(document: &Document) -> Vec<ResourceReference> {
    let mut out = Vec::new();
    walk::elements(document, |element| collect_link(element, &mut out));
    out
}

fn collect_link(element: &Element, out: &mut Vec<ResourceReference>) {
    let Some(kind) = link_kind(element) else {
        return;
    };
    if let Some(url) = element
        .attrs
        .get("href")
        .filter(|url| !url.trim().is_empty())
    {
        out.push(ResourceReference {
            kind,
            url: url.trim().into(),
        });
    }
}

fn link_kind(element: &Element) -> Option<ResourceKind> {
    if element.tag != "link" {
        return None;
    }
    if rel_has(element, "modulepreload") {
        return Some(ResourceKind::Script);
    }
    if !rel_has(element, "preload") {
        return None;
    }
    match element.attrs.get("as").map(|value| value.trim()) {
        Some(value) if value.eq_ignore_ascii_case("script") => Some(ResourceKind::Script),
        Some(value) if value.eq_ignore_ascii_case("style") => Some(ResourceKind::Stylesheet),
        Some(value) if value.eq_ignore_ascii_case("image") => Some(ResourceKind::Image),
        _ => None,
    }
}

fn rel_has(element: &Element, target: &str) -> bool {
    element.attrs.get("rel").is_some_and(|rel| {
        rel.split_whitespace()
            .any(|part| part.eq_ignore_ascii_case(target))
    })
}
