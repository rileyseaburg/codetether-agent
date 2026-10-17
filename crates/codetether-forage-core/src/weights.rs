//! Shared forage weights rules.
use chrono::{DateTime, Utc};
use codetether_okr::OkrStatus;
/// Return the score multiplier for an objective lifecycle status.
///
/// # Arguments
/// * `status` — Objective status.
/// # Returns
/// Active/draft/on-hold priority, or zero for terminal states.
/// # Examples
/// ```
/// assert_eq!(codetether_forage_core::status_weight(codetether_okr::OkrStatus::Completed), 0.0);
/// ```
pub fn status_weight(status: OkrStatus) -> f64 {
    match status {
        OkrStatus::Active => 1.0,
        OkrStatus::Draft => 0.65,
        OkrStatus::OnHold => 0.35,
        OkrStatus::Completed | OkrStatus::Cancelled => 0.0,
    }
}

/// Score deadline urgency at a supplied instant.
///
/// # Arguments
/// * `target_date` — Optional objective deadline.
/// * `now` — Shared cycle timestamp.
/// # Returns
/// Overdue, week or month urgency bonus, otherwise zero.
/// # Examples
/// ```
/// assert_eq!(codetether_forage_core::urgency_bonus(None, chrono::Utc::now()), 0.0);
/// ```
pub fn urgency_bonus(target_date: Option<DateTime<Utc>>, now: DateTime<Utc>) -> f64 {
    let Some(target_date) = target_date else {
        return 0.0;
    };

    if target_date <= now {
        return 0.35;
    }

    let days = (target_date - now).num_days();
    if days <= 7 {
        0.25
    } else if days <= 30 {
        0.1
    } else {
        0.0
    }
}
