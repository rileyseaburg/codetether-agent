mod event;
mod item;
mod lifecycle;
mod patch;
mod thread;
mod thread_approval;
mod thread_command;
mod thread_patch;
mod thread_tool;
mod tool;
mod writer;

use crate::provider::Usage;

pub(super) use lifecycle::{write_completed, write_failed, write_failed_response, write_started};
#[allow(unused_imports)]
pub(super) use {
    item::{write_item_completed, write_item_started},
    patch::write_patch_approval_required,
    thread::write_thread_event_to,
    tool::{write_tool_call_completed, write_tool_call_started},
};

pub(super) fn enabled(format: &str) -> bool {
    format == "jsonl"
}

pub(super) fn usage(usage: &Usage) -> Option<&Usage> {
    (usage.prompt_tokens != 0 || usage.completion_tokens != 0 || usage.total_tokens != 0)
        .then_some(usage)
}

#[cfg(test)]
mod tests;
