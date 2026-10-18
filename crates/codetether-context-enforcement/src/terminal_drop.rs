//! Application diagnostic for terminal truncation.

pub(super) fn report_fit(shrunk: usize, evicted: usize, target_tokens: usize) {
    tracing::warn!(target: "codetether_agent::session::helper::compression::terminal_drop",
        shrunk,
        evicted,
        target_tokens,
        "Terminal truncation fit retained tail"
    );
}
