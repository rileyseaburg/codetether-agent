//! Application diagnostic for terminal truncation.

pub(super) fn report_fit(shrunk: usize, evicted: usize, target_tokens: usize) {
    tracing::warn!(
        shrunk,
        evicted,
        target_tokens,
        "Terminal truncation fit retained tail"
    );
}
