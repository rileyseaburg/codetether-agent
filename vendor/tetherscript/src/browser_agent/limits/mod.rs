//! Resource limits for agent-controlled browser pages.
//!
//! The limits module stores deterministic guard settings and current page
//! guard metadata. Enforcement is intentionally small: page runtime entry
//! points check DOM size before executing JavaScript.

mod checks;
mod metadata;
mod model;
mod page;

pub use metadata::BrowserGuardMetadata;
pub use model::BrowserResourceLimits;
