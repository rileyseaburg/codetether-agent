//! Frontend-independent question editing and answer commitment.
//!
//! [Draft] owns selection state; [Action] edits the current question. Submission
//! validates that answer before advancing and returns the final ordered batch.
mod action;
mod edit;
mod state;
mod submit;
mod view;
pub use action::Action;
pub use state::Draft;
