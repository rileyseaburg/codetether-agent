//! Agent automation helpers for the deterministic browser.
//!
//! The module wraps [`BrowserSession`](crate::browser_session::BrowserSession)
//! with small page, locator, and action primitives that are useful to agents.

pub mod action;
mod action_checks;
pub mod actionability;
pub mod assertions;
mod clipboard_action;
pub mod context;
pub mod dialog;
pub mod diff;
pub mod downloads;
mod editable;
pub mod events;
pub mod frames;
pub mod hit;
pub mod hit_layout;
mod hit_style;
mod hit_target;
mod idrefs;
pub mod interact;
pub mod keyboard;
mod keyboard_action;
mod keyboard_escape;
mod labels;
pub mod limits;
pub mod locator;
mod locator_debug;
mod locator_filters;
mod locator_methods;
mod names;
pub mod navigation;
mod navigation_state;
pub mod network;
pub mod page;
pub mod permissions;
mod prepare;
pub mod query;
pub mod resolve;
mod retry;
mod roles;
mod runtime;
pub mod screenshot;
pub mod script;
pub mod scroll;
mod selector_ext;
mod text_match;
pub mod trace;
pub mod understanding;
pub mod wait;
mod wait_options;

mod exports;
#[cfg(test)]
mod test_modules;

pub use exports::*;
