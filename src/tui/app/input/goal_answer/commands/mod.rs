//! Native goal/task input bindings and integration tests.
mod input_host;
#[cfg(test)]
use codetether_goal_command::input as pending;
mod restart_host;
mod resume;
mod resume_host;
use crate::tui::app::{
    session_runtime::{SessionSlot, TuiSessionHandle},
    state::App,
};
pub(crate) use resume::{drain, restart};
pub(crate) async fn intercept(
    app: &mut App,
    slot: &SessionSlot,
    runtime: &TuiSessionHandle,
) -> bool {
    codetether_goal_command::input::intercept(&mut input_host::Native { app, slot, runtime }).await
}
#[cfg(test)]
mod tests;
