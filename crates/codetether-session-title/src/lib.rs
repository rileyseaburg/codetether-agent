//! Consistent session title derivation, cleanup and model requests.
//!
//! [normalize_title] and [label] sanitize stored and displayed names.
//! [TitleSession] supplies native field writes and timestamp updates for the
//! shared lifecycle. [generate_with] uses a caller-selected provider/model.
mod ai;
mod ai_model;
mod ai_request;
mod display;
mod extract;
mod host;
mod lifecycle;
mod seed;
pub use ai::generate_with;
pub use ai_model::title_from_model;
pub use display::{label, normalize_title};
pub use extract::is_title_candidate;
pub use host::TitleSession;
pub use lifecycle::{clear, context_change, generate, regenerate, set};
pub use seed::first_user_text;
#[cfg(test)]
mod display_tests;
