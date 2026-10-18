//! Session opt-in and environment policy for automatic confirmation.
//! Headless edit auto-apply policy.
//!
//! Edit tools emit *preview* results that normally require confirmation. This
//! module decides whether a pending edit should be applied without a human in
//! the loop — either because the session opted in, or because a non-interactive
//! SWE-bench / Terminal-Bench run set `CODETETHER_AUTO_APPLY_EDITS`.

/// Returns `true` when pending edits should auto-apply: either the session
/// opted in through `session_opt_in`, or the
/// `CODETETHER_AUTO_APPLY_EDITS` environment variable requests it.
///
/// # Arguments
/// `session_opt_in` is the session's automatic edit confirmation setting.
/// # Returns
/// True for an opted-in session or a nonempty environment setting other than
/// `0` or case-insensitive `false`.
/// # Examples
/// ```
/// assert!(codetether_edit_confirmation::auto_apply_enabled(true));
/// ```
pub fn auto_apply_enabled(session_opt_in: bool) -> bool {
    session_opt_in
        || std::env::var("CODETETHER_AUTO_APPLY_EDITS")
            .map(|v| !v.is_empty() && v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(false)
}
