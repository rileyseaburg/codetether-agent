//! Locator descriptions for agent browser actions.

/// Supported locator strategies.
#[derive(Clone, PartialEq, Eq)]
pub enum LocatorKind {
    /// CSS selector locator.
    Css(String),
    /// Visible text substring locator.
    Text(String),
    /// Visible text exact locator.
    TextExact(String),
    /// Explicit `role` attribute locator.
    Role(String),
    /// Role locator constrained by accessible name.
    RoleName { role: String, name: String },
    /// `data-testid` attribute locator.
    TestId(String),
    /// Form-control label locator.
    Label(String),
    /// Placeholder attribute locator.
    Placeholder(String),
    /// Image alt-text locator.
    AltText(String),
    /// Title attribute locator.
    Title(String),
}

/// A strict element locator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locator {
    pub kind: LocatorKind,
    pub strict: bool,
}
