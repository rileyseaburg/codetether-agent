//! Text proximity strategy.

use super::super::{DomFingerprint, DomNode, HealStrategy, SelectorCandidate};

pub fn text_proximity(root: &DomNode, fp: &DomFingerprint) -> Vec<SelectorCandidate> {
    let mut all = Vec::new();
    root.walk(&mut all, vec![]);
    all.into_iter()
        .filter_map(|(p, n)| {
            let f = DomFingerprint::from_dom(root, &p)?;
            (f.tag == fp.tag && f.text_hash == fp.text_hash).then(|| {
                SelectorCandidate::new(
                    format!("{}:text(\"{}\")", n.tag, n.text.trim()),
                    0.75,
                    HealStrategy::TextProximity,
                    vec![p],
                )
            })
        })
        .collect()
}
