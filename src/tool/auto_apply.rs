//! Headless edit auto-apply policy.
//!
//! Edit tools emit *preview* results that normally require confirmation. This
//! module decides whether a pending edit should be applied without a human in
//! the loop — either because the session opted in, or because a non-interactive
//! SWE-bench / Terminal-Bench run set `CODETETHER_AUTO_APPLY_EDITS`.

/// Returns `true` when pending edits should auto-apply: either the session
/// opted in via [`SessionMetadata::auto_apply_edits`], or the
/// `CODETETHER_AUTO_APPLY_EDITS` environment variable requests it.
///
/// [`SessionMetadata::auto_apply_edits`]: crate::session::SessionMetadata::auto_apply_edits
pub fn auto_apply_enabled(meta: &crate::session::SessionMetadata) -> bool {
    meta.auto_apply_edits
        || std::env::var("CODETETHER_AUTO_APPLY_EDITS")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(false)
}
