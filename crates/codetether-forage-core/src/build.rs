//! Construct one eligible scored opportunity without host execution.
use crate::{
    ForageOpportunity, MoonshotRubric, alignment::moonshot_alignment_score,
    hits::matching_moonshots, prompt::build_execution_prompt, weights::urgency_bonus,
};
use chrono::{DateTime, Utc};
use codetether_okr::{KeyResult, Okr};
pub(super) fn build(
    okr: &Okr,
    kr: &KeyResult,
    moonshots: &MoonshotRubric,
    now: DateTime<Utc>,
    status_weight: f64,
    score: &mut dyn FnMut(&Okr, &KeyResult, f64, f64, f64) -> f64,
) -> Option<ForageOpportunity> {
    let progress = kr.progress().clamp(0.0, 1.0);
    if progress >= 1.0 {
        return None;
    }
    let remaining = (1.0 - progress).clamp(0.0, 1.0);
    let urgency_bonus = urgency_bonus(okr.target_date, now);
    let alignment_context = format!(
        "{} {} {} {}",
        okr.title, okr.description, kr.title, kr.description
    );
    let moonshot_alignment = moonshot_alignment_score(&alignment_context, &moonshots.goals);
    if moonshots.required && moonshot_alignment < moonshots.min_alignment {
        return None;
    }
    let moonshot_hits = matching_moonshots(&alignment_context, &moonshots.goals);
    let moonshot_bonus = if moonshots.goals.is_empty() {
        0.0
    } else {
        (moonshot_alignment * 0.5).min(0.5)
    };
    let score = score(
        okr,
        kr,
        (remaining * status_weight) + urgency_bonus + moonshot_bonus,
        remaining,
        moonshot_alignment,
    );
    let prompt = build_execution_prompt(okr, kr, moonshots, moonshot_alignment);

    Some(crate::record::build(
        okr,
        kr,
        (score, progress, remaining),
        (moonshot_alignment, moonshot_hits),
        prompt,
    ))
}
