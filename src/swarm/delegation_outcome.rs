//! Phase C step 28 — record swarm subtask outcomes into the
//! [`DelegationState`] sidecar so the next dispatch's LCB ranking sees
//! evidence from the prior runs.
//!
//! The dispatch side lives in [`super::delegation::choose_provider_for_subtask`].

use std::sync::{Arc, Mutex};

use crate::session::delegation::DelegationState;
use crate::session::delegation_skills::SWARM_DISPATCH;
use crate::session::relevance::{Bucket, Dependency, Difficulty, ToolUse};

/// Bucket key used by the swarm dispatch hook.
///
/// Mirrors `super::delegation::choose_provider_for_subtask` — keeping the
/// projection in one place stops update/score sites from drifting onto
/// different bucket keys and silently fragmenting the posterior.
pub fn bucket_for_specialty(specialty: &str) -> Bucket {
    let s = specialty.to_ascii_lowercase();
    let difficulty = if s.contains("security") || s.contains("architect") {
        Difficulty::Hard
    } else if s.contains("review") || s.contains("test") {
        Difficulty::Medium
    } else {
        Difficulty::Easy
    };
    let tool_use = if s.contains("deploy") || s.contains("infra") || s.contains("debug") {
        ToolUse::Yes
    } else {
        ToolUse::No
    };
    Bucket {
        difficulty,
        dependency: Dependency::Isolated,
        tool_use,
    }
}

/// Record `(provider, specialty, success)` against the shared sidecar.
///
/// No-op when delegation is disabled. Locking failure is logged and
/// otherwise ignored — losing one update never justifies crashing the
/// swarm loop.
pub fn record_subtask_outcome(
    state: &Arc<Mutex<DelegationState>>,
    provider: &str,
    specialty: &str,
    success: bool,
) {
    let bucket = bucket_for_specialty(specialty);
    match state.lock() {
        Ok(mut guard) => {
            if !guard.enabled() {
                return;
            }
            guard.update(provider, SWARM_DISPATCH, bucket, success);
        }
        Err(err) => {
            tracing::warn!(%err, "delegation state mutex poisoned; skipping swarm outcome update");
        }
    }
}
