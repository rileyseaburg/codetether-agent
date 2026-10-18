//! Ralph run state, event reduction and terminal presentation.
//!
//! [RalphViewState] owns stories, selection and the optional event receiver.
//! Attach a receiver, drain its events, and use the progress/navigation methods
//! to present the run. Events use the shared [RalphEvent] schema.
//! [render_ralph_view] draws either the overview or the selected story details.
pub use codetether_ralph_core::events::{RalphEvent, RalphStoryInfo, RalphStoryStatus};
mod detail_scroll;
mod event_rx;
mod events;
mod lifecycle;
mod navigation;
mod progress;
mod run_end;
mod run_start;
mod state;
mod story_activity;
mod story_checks;
mod story_results;
pub use state::RalphViewState;

mod story_messages;

mod view;
pub use view::render_ralph_view;
