//! Move-based TUI session runtime.

mod active_cancel;
#[cfg(test)]
mod active_turn;
mod command;
mod execute;
mod execute_host;
mod handle;
mod dispatch_host;
mod loop_submit;
mod notice;
mod prompt_result;
mod runtime;
mod slot;
mod view;

#[cfg(test)]
#[path = "runtime_completion_tests.rs"]
mod completion_tests;
#[cfg(test)]
mod tests;

pub(crate) use command::{PromptRequest, SessionCommand};
pub(crate) use handle::TuiSessionHandle;
pub(crate) use notice::SessionNotice;
pub(crate) use runtime::spawn;
pub(crate) use slot::SessionSlot;
pub use view::SessionView;
