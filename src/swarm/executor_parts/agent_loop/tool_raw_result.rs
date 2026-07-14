//! Outcome of one raw tool invocation.

pub(super) struct Result {
    pub output: String,
    pub success: bool,
    pub timed_out: bool,
}
