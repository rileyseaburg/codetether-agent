//! Move-based TUI session runtime.

mod command;
mod execute;
mod handle;
mod loop_step;
mod loop_submit;
mod notice;
mod prompt_result;
mod runtime;
mod slot;
mod view;

#[cfg(test)]
mod tests;

pub(crate) use command::{PromptRequest, SessionCommand};
pub(crate) use handle::TuiSessionHandle;
pub(crate) use notice::SessionNotice;
pub(crate) use runtime::spawn;
pub(crate) use slot::SessionSlot;
pub use view::SessionView;
