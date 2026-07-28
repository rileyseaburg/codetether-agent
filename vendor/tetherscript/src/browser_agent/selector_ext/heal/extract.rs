//! Element property extraction trait.

use super::score::ElementProps;

/// Trait for extracting element properties for selector healing.
pub trait ElementLike {
    fn tag_name(&self) -> String;
    fn attr(&self, name: &str) -> Option<String>;
    fn classes(&self) -> Vec<String>;
    fn text(&self) -> Option<String>;
    fn parent_tag(&self) -> Option<String> {
        None
    }
    fn position_hint(&self) -> Option<usize> {
        None
    }
}

/// Extract a property fingerprint from any ElementLike.
pub fn extract_props<E: ElementLike>(el: &E) -> ElementProps {
    ElementProps {
        tag: el.tag_name(),
        id: el.attr("id"),
        classes: el.classes(),
        role: el.attr("role"),
        text: el.text(),
        href: el.attr("href"),
        label: el.attr("aria-label").or_else(|| el.attr("title")),
        parent_tag: el.parent_tag(),
        position_hint: el.position_hint(),
    }
}
