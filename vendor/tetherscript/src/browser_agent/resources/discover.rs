//! DOM scanning for external resource references.

use crate::browser::{Document, Element};

use super::{walk, ResourceKind};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ResourceReference {
    pub(crate) kind: ResourceKind,
    pub(crate) url: String,
}

pub(crate) fn collect(document: &Document) -> Vec<ResourceReference> {
    let mut out = Vec::new();
    walk::elements(document, |element| collect_element(element, &mut out));
    out
}

fn collect_element(element: &Element, out: &mut Vec<ResourceReference>) {
    match element.tag.as_str() {
        "script" => push_attr(out, ResourceKind::Script, element, "src"),
        "img" => push_attr(out, ResourceKind::Image, element, "src"),
        "link" if is_stylesheet(element) => {
            push_attr(out, ResourceKind::Stylesheet, element, "href");
        }
        _ => {}
    }
}

fn push_attr(out: &mut Vec<ResourceReference>, kind: ResourceKind, element: &Element, name: &str) {
    if let Some(url) = element.attrs.get(name).filter(|url| !url.trim().is_empty()) {
        out.push(ResourceReference {
            kind,
            url: url.trim().into(),
        });
    }
}

fn is_stylesheet(element: &Element) -> bool {
    element
        .attrs
        .get("rel")
        .is_some_and(|rel| rel.split_whitespace().any(|part| part == "stylesheet"))
}
