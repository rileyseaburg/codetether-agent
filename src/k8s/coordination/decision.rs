//! Pure lease ownership and expiry decisions.

use chrono::{DateTime, Duration, Utc};

/// Existing lease facts needed for conflict resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseState {
    pub holder: Option<String>,
    pub renewed_at: Option<DateTime<Utc>>,
    pub duration_seconds: i32,
}

/// Atomic action selected from the observed lease state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeaseDecision {
    Acquire,
    Renew,
    TakeOver,
    Conflict {
        holder: String,
        expires_at: DateTime<Utc>,
    },
}

pub fn owns(state: &LeaseState, claimant: &str) -> bool {
    state.holder.as_deref() == Some(claimant)
}

pub fn decide(state: Option<&LeaseState>, claimant: &str, now: DateTime<Utc>) -> LeaseDecision {
    let Some(state) = state else {
        return LeaseDecision::Acquire;
    };
    if state.holder.as_deref() == Some(claimant) {
        return LeaseDecision::Renew;
    }
    let renewed = state.renewed_at.unwrap_or(DateTime::<Utc>::UNIX_EPOCH);
    let expires_at = renewed + Duration::seconds(i64::from(state.duration_seconds.max(1)));
    if now >= expires_at {
        LeaseDecision::TakeOver
    } else {
        LeaseDecision::Conflict {
            holder: state.holder.clone().unwrap_or_else(|| "unknown".into()),
            expires_at,
        }
    }
}
