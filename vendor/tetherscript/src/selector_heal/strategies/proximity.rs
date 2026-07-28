//! Proximity and position hint strategies.

use super::super::{DomNode, HealStrategy, SelectorCandidate};

pub fn sibling_proximity(root: &DomNode, path: &[usize]) -> Vec<SelectorCandidate> {
    if path.is_empty() {
        return vec![];
    }
    let mut parent = path.to_vec();
    let idx = parent.pop().unwrap();
    let Some(p) = root.get(&parent) else {
        return vec![];
    };
    let tag = p
        .children
        .get(idx)
        .map(|n| n.tag.clone())
        .unwrap_or_default();
    vec![SelectorCandidate::new(
        format!("{}:near-sibling({})", tag, idx),
        0.38,
        HealStrategy::SiblingProximity,
        vec![path.to_vec()],
    )]
}

pub fn position_hint(root: &DomNode, path: &[usize]) -> Vec<SelectorCandidate> {
    let Some(n) = root.get(path) else {
        return vec![];
    };
    let pos = path.last().copied().unwrap_or(0) + 1;
    vec![SelectorCandidate::new(
        format!("the {}th {} in its parent", pos, n.tag),
        0.25,
        HealStrategy::PositionHint,
        vec![path.to_vec()],
    )]
}
