//! Run command JSONL output and durable thread event adapters.
//!
//! Emit lifecycle records with [write_started] and [write_completed], or map
//! durable events with [write_thread_event_to]. Each emitted record is one
//! newline-terminated JSON object. Unsupported thread events produce no output.
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

use codetether_provider_core::Usage;

pub use lifecycle::{write_completed, write_failed, write_failed_response, write_started};
pub use thread::codex_thread::write_codex_thread_event_to;
#[allow(unused_imports)]
pub use {
    item::{write_item_completed, write_item_started},
    patch::write_patch_approval_required,
    thread::write_thread_event_to,
    tool::{write_tool_call_completed, write_tool_call_started},
};

/// Return whether the requested format enables run JSONL.
/// # Arguments
/// * `format` - CLI output format.
/// # Returns
/// True only for the exact `jsonl` spelling.
/// # Examples
/// ```
/// assert!(codetether_run_jsonl::enabled("jsonl"));
/// assert!(!codetether_run_jsonl::enabled("json"));
/// ```
pub fn enabled(format: &str) -> bool {
    format == "jsonl"
}

/// Omit usage when all three token counts are zero.
/// # Arguments
/// * `usage` - Provider usage to inspect.
/// # Returns
/// The borrowed usage when any token count is nonzero.
/// # Examples
/// ```
/// assert!(codetether_run_jsonl::usage(&codetether_provider_core::Usage::default()).is_none());
/// ```
pub fn usage(usage: &Usage) -> Option<&Usage> {
    (usage.prompt_tokens != 0 || usage.completion_tokens != 0 || usage.total_tokens != 0)
        .then_some(usage)
}

#[cfg(test)]
mod tests;
