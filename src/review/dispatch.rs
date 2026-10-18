//! Bind shared background dispatch to native reviewer configuration and journals.
use super::host::Native;
pub(crate) use codetether_approval_review::dispatch::Context;
pub(crate) fn start(
    item: &codetether_approval_queue::ApprovalSnapshot,
    context: impl FnOnce() -> Context,
) {
    codetether_approval_review::dispatch::start::<Native>(item, context);
}
