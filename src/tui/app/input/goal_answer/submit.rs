//! Native entry point for shared question/continuation interception.
use crate::tui::app::{
    session_runtime::{SessionSlot, TuiSessionHandle},
    state::App,
};
pub(crate) async fn intercept(
    app: &mut App,
    slot: &SessionSlot,
    runtime: &TuiSessionHandle,
) -> bool {
    codetether_goal_answer::intercept(&mut super::input_host::Native { app, slot, runtime }).await
}
#[cfg(test)]
#[path = "submit_followup_tests.rs"]
mod followup_tests;
#[cfg(test)]
#[path = "submit_tests.rs"]
mod tests;
