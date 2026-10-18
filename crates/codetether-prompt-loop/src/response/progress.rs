//! File-writing progress counters and corrective-message sequencing.
use crate::{LoopState, driver::ParsedCall};
use codetether_prompt_policy::loop_constants as limits;
use codetether_provider_core::Message;

/// State and transcript access required by write-progress tracking.
/// # Implementors
/// Native runners expose their shared counters and existing message insertion.
pub trait Host {
    /// Borrow the current turn's progress counters.
    fn progress(&mut self) -> &mut LoopState;
    /// Append corrective guidance through the existing session path.
    fn add_message(&mut self, message: Message);
}

/// Track file-writing tool calls and nudge a stalled agent.
///
/// # Arguments
/// * `host` — Turn counters and transcript insertion.
/// * `step` — Step number included in the progress warning.
/// * `calls` — Parsed calls for the current provider response.
/// # Returns
/// Updates the counter, emits guidance at the existing threshold, then resets it.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_prompt_loop::response::progress::Host) {
/// codetether_prompt_loop::response::progress::track_writes(host, 1, &[]);
/// # }
/// ```
pub fn track_writes(host: &mut impl Host, step: usize, calls: &[ParsedCall]) {
    let wrote = calls.iter().any(|(_, name, _)| {
        matches!(
            name.as_str(),
            "write" | "edit" | "create_file" | "replace_string_in_file" | "edit_file" | "bash"
        )
    });
    if wrote {
        host.progress().steps_since_write = 0;
    } else {
        host.progress().steps_since_write += 1;
    }
    if host.progress().steps_since_write < limits::MAX_STEPS_WITHOUT_PROGRESS {
        return;
    }
    tracing::warn!(
        target: "codetether_agent::session::helper::prompt_loop::response::progress",
        step,
        steps_since_last_write = host.progress().steps_since_write,
        "No file-writing tools called recently"
    );
    host.add_message(super::corrective_message(limits::NO_PROGRESS_NUDGE));
    host.progress().steps_since_write = 0;
}
