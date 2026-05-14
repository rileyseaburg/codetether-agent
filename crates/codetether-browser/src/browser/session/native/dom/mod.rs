//! DOM interaction handlers for native browser pages.

mod forms;
mod input;
mod motion;
mod page;
mod selector;
mod upload;

/// Form helpers.
pub(super) use forms::{click_text, fill, toggle};
/// Text and key input helpers.
pub(super) use input::{press, type_text};
/// Pointer, focus, and scroll helpers.
pub(super) use motion::{blur, click, focus, hover, scroll};
/// File upload helper.
pub(super) use upload::upload;
