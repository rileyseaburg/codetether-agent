//! Native trace observation, event projection and approval handling.
use crate::session::SessionEvent;
use codetether_child_events::{Event, Host};
/// Native session-event operations used by the shared collector.
pub struct ApplicationHost;
impl Host for ApplicationHost {
    type SourceEvent = SessionEvent;
    type Approval = crate::approval::LiveApprovalRequest;
    fn observe(agent: &str, event: &SessionEvent) {
        super::live_trace::observe(agent, event);
    }
    fn project(event: SessionEvent) -> Event<Self::Approval> {
        match event {
            SessionEvent::TextComplete(text) => Event::TextComplete(text),
            SessionEvent::ThinkingComplete(text) => Event::ThinkingComplete(text),
            SessionEvent::ToolCallComplete {
                name,
                output,
                success,
                ..
            } => Event::ToolCallComplete {
                name,
                output,
                success,
            },
            SessionEvent::Error(error) => Event::Error(error),
            SessionEvent::ApprovalRequest(request) => Event::ApprovalRequest(request),
            SessionEvent::Done => Event::Done,
            _ => Event::Ignored,
        }
    }
    fn approve(request: &Self::Approval) {
        super::approve::auto_approve(request);
    }
}
