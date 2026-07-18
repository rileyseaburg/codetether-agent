//! Mock providers used by whole-request restart tests.

#[path = "restart_test_provider/eof.rs"]
mod eof;
#[path = "restart_test_provider/fault.rs"]
mod fault;

pub(super) use eof::FlakyThenCompleteProvider;
pub(super) use fault::TransientFaultThenCompleteProvider;
