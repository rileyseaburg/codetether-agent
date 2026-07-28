//! Role based strategy.

use super::super::{DomNode, HealStrategy, SelectorCandidate, SelectorGenerator};

pub fn role_based(root: &DomNode, path: &[usize], g: &SelectorGenerator) -> Vec<SelectorCandidate> {
    let Some(n) = root.get(path) else {
        return vec![];
    };
    let Some(role) = n.attr("role") else {
        return vec![];
    };
    let s = format!("[role=\"{}\"][aria-label=\"{}\"]", role, n.name());
    let m = g.find(root, &s);
    vec![SelectorCandidate::new(s, 0.82, HealStrategy::RoleBased, m)]
}
