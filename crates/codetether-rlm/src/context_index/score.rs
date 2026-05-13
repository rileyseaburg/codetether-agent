//! Evidence scoring.

use super::{EvidenceKind, EvidenceRecord, PlanIntent, RetrievalPlan};

/// Score one record and return a human-readable selection reason.
pub(super) fn record(record: &EvidenceRecord, plan: &RetrievalPlan) -> (f32, String) {
    let hay = format!(
        "{} {} {}",
        record.source,
        record.symbols.join(" "),
        record.text
    )
    .to_lowercase();
    let hits = plan
        .terms
        .iter()
        .filter(|term| hay.contains(term.as_str()))
        .count();
    let boost = kind_boost(record.kind.clone()) + intent_boost(record.kind.clone(), plan.intent);
    let symbol_hit = record
        .symbols
        .iter()
        .any(|s| plan.terms.iter().any(|t| s.to_lowercase().contains(t)));
    let value = hits as f32 * 4.0 + boost + if symbol_hit { 4.0 } else { 0.0 };
    let reason = if hits > 0 {
        format!("matched {hits} query terms")
    } else {
        format!("{:?} context fallback", record.kind)
    };
    (value, reason)
}

fn kind_boost(kind: EvidenceKind) -> f32 {
    match kind {
        EvidenceKind::Error => 3.0,
        EvidenceKind::Symbol => 2.0,
        EvidenceKind::Import => 1.0,
        EvidenceKind::Text => 0.2,
    }
}

fn intent_boost(kind: EvidenceKind, intent: PlanIntent) -> f32 {
    match (intent, kind) {
        (PlanIntent::Debug, EvidenceKind::Error) => 4.0,
        (PlanIntent::Symbol, EvidenceKind::Symbol) => 3.0,
        (PlanIntent::Summary, EvidenceKind::Text) => 0.8,
        _ => 0.0,
    }
}
