type SessionEvent = codetether_session_events::SessionEvent<()>;
use crate::{ThreadEventContext, ThreadEventMapper};

mod approval;
mod approval_decision;
mod command;
mod item;
mod lifecycle;
mod patch;
mod retry;
mod stable_item;
mod tool;
mod tool_metadata;
mod tool_output;

fn mapper() -> ThreadEventMapper {
    ThreadEventMapper::with_timestamp(ThreadEventContext::new("session-1", "turn-1"), 99)
}
