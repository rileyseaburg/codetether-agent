//! Compression-trigger selection from measured request cost and history size.

/// Request measurements and the resulting compression trigger.
///
/// # Examples
/// ```
/// use codetether_context_policy::compression::budget::check_budget;
/// let check = check_budget(100, 90, 60, 10, 10);
/// assert_eq!(check.trigger_reason, Some("history_length"));
/// ```
pub struct BudgetCheck {
    /// Model context window supplied by the host.
    pub ctx_window: usize,
    /// Maximum usable request budget supplied by the host.
    pub safety_budget: usize,
    /// Measured initial request cost, including prompt and tool overhead.
    pub initial_est: usize,
    /// None when compaction is unnecessary, otherwise its stable reason label.
    pub trigger_reason: Option<&'static str>,
}

/// Choose whether to compress for history length or context budget.
///
/// # Arguments
/// * `ctx_window` - Full model context window, retained in the result.
/// * `safety_budget` - Usable full-request token budget.
/// * `initial_est` - Measured request cost before compression.
/// * `history_trigger` - Message threshold; zero disables the history trigger.
/// * `message_count` - Current history length.
///
/// # Returns
/// Measurements and the selected trigger; budget overruns take precedence.
///
/// # Examples
/// ```
/// use codetether_context_policy::compression::budget::check_budget;
/// assert_eq!(check_budget(100, 90, 91, 0, 2).trigger_reason, Some("context_budget"));
/// assert_eq!(check_budget(100, 90, 90, 0, 2).trigger_reason, None);
/// ```
pub fn check_budget(
    ctx_window: usize,
    safety_budget: usize,
    initial_est: usize,
    history_trigger: usize,
    message_count: usize,
) -> BudgetCheck {
    let history_over = history_trigger > 0 && message_count >= history_trigger;

    let trigger_reason = if initial_est <= safety_budget && !history_over {
        None
    } else if history_over && initial_est <= safety_budget {
        Some("history_length")
    } else {
        Some("context_budget")
    };
    BudgetCheck {
        ctx_window,
        safety_budget,
        initial_est,
        trigger_reason,
    }
}
