use crate::PolicyRunResult;

pub(super) fn sample(
    policy: &'static str,
    tokens: usize,
    faults: usize,
    gap: usize,
    reuse: f64,
) -> PolicyRunResult {
    PolicyRunResult {
        policy,
        kept_messages: 0,
        context_tokens: tokens,
        fault_count: faults,
        oracle_gap: gap,
        reuse_rate: reuse,
    }
}
