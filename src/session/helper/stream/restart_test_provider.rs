//! Mock providers used by whole-request restart tests.

#[path = "restart_test_provider/eof.rs"]
mod eof;
#[path = "restart_test_provider/fault.rs"]
mod fault;
#[path = "restart_test_provider/retry_after.rs"]
mod retry_after;

pub(super) use eof::FlakyThenCompleteProvider;
pub(super) use fault::TransientFaultThenCompleteProvider;
pub(super) use retry_after::RetryAfterProvider;
