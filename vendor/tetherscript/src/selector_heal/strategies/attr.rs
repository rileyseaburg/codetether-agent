//! Attribute recovery strategy.

use super::super::{DomNode, HealStrategy, SelectorCandidate, SelectorGenerator};

pub fn attr_recovery(
    root: &DomNode,
    path: &[usize],
    g: &SelectorGenerator,
) -> Vec<SelectorCandidate> {
    g.generate(root, path)
        .into_iter()
        .map(|s| {
            let m = g.find(root, &s);
            let score = if m.len() == 1 { 0.9 } else { 0.55 };
            SelectorCandidate::new(s, score, HealStrategy::AttrRecovery, m)
        })
        .collect()
}
