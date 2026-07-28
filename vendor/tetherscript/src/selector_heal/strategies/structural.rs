//! Structural path strategy.

use super::super::{DomNode, HealStrategy, SelectorCandidate, SelectorGenerator};

pub fn structural_path(
    root: &DomNode,
    path: &[usize],
    g: &SelectorGenerator,
) -> Vec<SelectorCandidate> {
    let s = g.generate(root, path).last().cloned().unwrap_or_default();
    vec![SelectorCandidate::new(
        s,
        0.45,
        HealStrategy::StructuralPath,
        vec![path.to_vec()],
    )]
}
