//! Relay OKR templates, model drafting and response normalization.
//!
//! draft uses a caller-selected provider; okr_from_planned_draft validates its
//! schema values and falls back to default_relay_okr_template when needed.
mod convert;
mod draft;
mod parse;
mod planned;
mod prompt;
mod request;
mod template;
pub use convert::okr_from_planned_draft;
pub use draft::draft;
pub use planned::PlannedOkrDraft;
pub use template::default_relay_okr_template;
