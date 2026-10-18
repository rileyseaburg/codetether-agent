//! Startup history and policy notices.
use super::{Host, Session};
use codetether_session_resume::SessionLoadOutcome;
pub(super) fn session_banner<S: Session>(outcome: &SessionLoadOutcome, host: &mut impl Host<S>) {
    if let Some(message) = outcome.banner() {
        host.push_system(message);
        host.scroll_to_bottom();
    }
}
pub(super) fn policy_notice<S: Session>(host: &mut impl Host<S>, policy: &str) {
    let content = format!("Execution policy: {policy}");
    host.push_system(content);
    host.scroll_to_bottom();
}
