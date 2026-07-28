//! Locator matching predicates.

use crate::browser::{text_content, Document, Element, Node};
use crate::browser_agent::locator::{Locator, LocatorKind};
use crate::browser_agent::{names, roles, selector_ext, text_match};

pub(crate) fn matches_locator(
    document: &Document,
    node: &Node,
    element: &Element,
    ancestors: &[Element],
    locator: &Locator,
) -> bool {
    match &locator.kind {
        LocatorKind::Css(selector) => {
            selector_ext::matches(document, node, element, ancestors, selector)
        }
        LocatorKind::Text(text) => text_match::contains(&text_content(node), text),
        LocatorKind::TextExact(text) => text_match::exact(&text_content(node), text),
        LocatorKind::Role(role) => role_eq(element, role),
        LocatorKind::RoleName { role, name } => {
            role_eq(element, role)
                && text_match::contains(
                    &names::accessible_name(document, node, element, ancestors),
                    name,
                )
        }
        LocatorKind::TestId(id) => attr_eq(element, "data-testid", id),
        LocatorKind::Label(label) => text_match::contains(
            &names::label_name(document, element, ancestors).unwrap_or_default(),
            label,
        ),
        LocatorKind::Placeholder(text) => attr_contains(element, "placeholder", text),
        LocatorKind::AltText(text) => attr_contains(element, "alt", text),
        LocatorKind::Title(text) => attr_contains(element, "title", text),
    }
}

fn role_eq(element: &Element, role: &str) -> bool {
    roles::role_of(element) == role.to_ascii_lowercase()
}

fn attr_eq(element: &Element, name: &str, expected: &str) -> bool {
    element
        .attrs
        .get(name)
        .is_some_and(|value| value == expected)
}

fn attr_contains(element: &Element, name: &str, expected: &str) -> bool {
    element
        .attrs
        .get(name)
        .is_some_and(|value| text_match::contains(value, expected))
}
