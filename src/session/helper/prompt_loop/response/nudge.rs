//! Insert shared corrective messages into the native session.
use super::super::Runner;
pub(in crate::session::helper::prompt_loop) fn add(runner: &mut Runner<'_>, text: &str) {
    runner
        .session
        .add_message(codetether_prompt_loop::response::corrective_message(text));
}
