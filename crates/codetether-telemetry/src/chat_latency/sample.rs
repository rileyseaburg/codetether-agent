//! One completed turn timing sample.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct TurnSample {
    pub(super) e2e_ms: u64,
    pub(super) ttft_ms: Option<u64>,
}
