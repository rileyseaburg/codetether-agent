//! Input types for the browserctl tool.

mod action;

#[rustfmt::skip]
mod params;

/// Parsed browserctl action.
pub(crate) use action::BrowserCtlAction;
/// Parsed browserctl input payload.
pub(crate) use params::BrowserCtlInput;
