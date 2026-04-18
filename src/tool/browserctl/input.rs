//! Input types for the browserctl tool.

mod action;

#[rustfmt::skip]
mod params;

pub(in crate::tool::browserctl) use action::BrowserCtlAction;
pub(in crate::tool::browserctl) use params::BrowserCtlInput;
