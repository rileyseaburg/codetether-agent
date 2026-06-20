//! PRD verifiability checks for the Ralph loop.
//!
//! A story is *verifiable* when Ralph has at least one way to confirm it: either
//! the story declares `verification_steps`, or the PRD declares quality-check
//! commands. Without either, Ralph would mark the story passed having run
//! nothing — the "Ralph not verifying the PRD" failure mode. This module logs a
//! loud warning for every such story so the gap is visible up front.

use super::types::Prd;
use tracing::warn;

/// Warn about stories that cannot be verified by Ralph.
///
/// Emits a warning per incomplete story that has no `verification_steps` when
/// quality checks are also unavailable (disabled, or none configured).
pub fn warn_unverifiable_stories(prd: &Prd, quality_checks_enabled: bool) {
    let quality_available = quality_checks_enabled && !prd.quality_checks.is_empty();
    for story in &prd.user_stories {
        if story.passes {
            continue;
        }
        if story.verification_steps.is_empty() && !quality_available {
            warn!(
                story_id = %story.id,
                title = %story.title,
                "Story has no verification_steps and no quality checks are configured; \
                 Ralph cannot verify it and will mark it passed without checking. Add \
                 verification_steps or quality_checks to the PRD."
            );
        }
    }
}
