//! Constructors for agent browser locators.

use crate::browser_agent::locator::{Locator, LocatorKind};

impl Locator {
    /// Build a strict CSS selector locator.
    pub fn css(selector: impl Into<String>) -> Self {
        build(LocatorKind::Css(selector.into()))
    }

    /// Build a strict text substring locator.
    pub fn text(text: impl Into<String>) -> Self {
        build(LocatorKind::Text(text.into()))
    }

    /// Build a strict exact text locator.
    pub fn text_exact(text: impl Into<String>) -> Self {
        build(LocatorKind::TextExact(text.into()))
    }

    /// Build a strict role locator.
    pub fn role(role: impl Into<String>) -> Self {
        build(LocatorKind::Role(role.into()))
    }

    /// Build a strict role locator constrained by accessible name.
    pub fn role_name(role: impl Into<String>, name: impl Into<String>) -> Self {
        build(LocatorKind::RoleName {
            role: role.into(),
            name: name.into(),
        })
    }

    /// Build a strict `data-testid` locator.
    pub fn test_id(id: impl Into<String>) -> Self {
        build(LocatorKind::TestId(id.into()))
    }

    /// Build a strict form-control label locator.
    pub fn label(label: impl Into<String>) -> Self {
        build(LocatorKind::Label(label.into()))
    }

    /// Build a strict placeholder locator.
    pub fn placeholder(text: impl Into<String>) -> Self {
        build(LocatorKind::Placeholder(text.into()))
    }

    /// Build a strict alt-text locator.
    pub fn alt_text(text: impl Into<String>) -> Self {
        build(LocatorKind::AltText(text.into()))
    }

    /// Build a strict title locator.
    pub fn title(text: impl Into<String>) -> Self {
        build(LocatorKind::Title(text.into()))
    }

    /// Allow the locator to resolve to the first matching element.
    pub fn relaxed(mut self) -> Self {
        self.strict = false;
        self
    }
}

fn build(kind: LocatorKind) -> Locator {
    Locator { kind, strict: true }
}
