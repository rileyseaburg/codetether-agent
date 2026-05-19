//! Input types for the browserctl tool.

mod action;

#[rustfmt::skip]
mod params;

/// Parsed browserctl action.
pub(in crate::tool::browserctl) use action::BrowserCtlAction;
/// Parsed browserctl input payload.
pub(in crate::tool::browserctl) use params::BrowserCtlInput;
