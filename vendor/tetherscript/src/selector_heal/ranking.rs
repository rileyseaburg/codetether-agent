//! Candidate confidence ranking.
use super::{DomFingerprint, DomNode, HealStrategy, SelectorCandidate};
use std::collections::HashMap;

/// Rank candidates by uniqueness, fingerprint similarity, and strategy prior.
pub fn rank_candidates(
    root: &DomNode,
    mut xs: Vec<SelectorCandidate>,
    fp: Option<&DomFingerprint>,
) -> Vec<SelectorCandidate> {
    for c in &mut xs {
        let unique = if c.matches.len() == 1 { 0.18 } else { 0.0 };
        let sim = fp
            .and_then(|f| {
                c.matches
                    .first()
                    .and_then(|p| DomFingerprint::from_dom(root, p))
                    .map(|g| f.similarity(&g) * 0.35)
            })
            .unwrap_or(0.0);
        c.confidence = (c.confidence + unique + sim + strategy_prior(&c.strategy)).clamp(0.0, 1.0);
    }
    xs.sort_by(|a, b| b.confidence.total_cmp(&a.confidence));
    dedup(xs)
}

fn strategy_prior(s: &HealStrategy) -> f32 {
    match s {
        HealStrategy::AttrRecovery => 0.07,
        HealStrategy::RoleBased => 0.06,
        HealStrategy::TextProximity => 0.04,
        HealStrategy::StructuralPath => 0.02,
        HealStrategy::SiblingProximity => 0.01,
        HealStrategy::PositionHint => 0.0,
        HealStrategy::Generated => 0.05,
    }
}

fn dedup(xs: Vec<SelectorCandidate>) -> Vec<SelectorCandidate> {
    let mut best: HashMap<String, SelectorCandidate> = HashMap::new();
    for x in xs {
        best.entry(x.selector.clone())
            .and_modify(|b| {
                if x.confidence > b.confidence {
                    *b = x.clone();
                }
            })
            .or_insert(x);
    }
    let mut v: Vec<_> = best.into_values().collect();
    v.sort_by(|a, b| b.confidence.total_cmp(&a.confidence));
    v
}
