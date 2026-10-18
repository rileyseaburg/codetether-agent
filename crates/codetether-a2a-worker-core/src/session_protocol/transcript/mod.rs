//! Worker transcript updates and persisted halt diagnostics.
//! [record_loop_halt] appends and saves a visible stop reason. [maybe_inject]
//! appends the budget wrap-up prompt; [add_tool_result] records raw tool replies.
//! [Host] supplies native transcript insertion and persistence.
mod halt;
mod host;
mod tool;
mod wrap_up;
pub use halt::record_loop_halt;
pub use host::Host;
pub use tool::add_tool_result;
pub use wrap_up::maybe_inject;
