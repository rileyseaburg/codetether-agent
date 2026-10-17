//! Prompt, handoff, learning and status rendering for Ralph.
//!
//! Functions accept shared contracts and text; callers supply application state
//! and progress content. Prompt assets preserve the original wording.
mod handoff;
mod learnings;
mod normalize;
mod progress_prompt;
mod status;
mod story_prompt;
mod verification_steps;
pub use handoff::prepare_relay_handoff;
pub use learnings::{extract_learnings, extract_learnings_static};
pub use normalize::normalize_for_relay;
pub use progress_prompt::build_prompt;
pub use status::status_markdown;
pub use story_prompt::build_story_prompt;
