//! Self-healing selector entry point.
use super::{ranking, strategies, DomFingerprint, DomNode, SelectorCandidate, SelectorGenerator};

/// Recovers likely replacement selectors when a selector breaks.
#[derive(Clone, Debug, Default)]
pub struct SelfHealingSelector {
    pub generator: SelectorGenerator,
}

impl SelfHealingSelector {
    pub fn new() -> Self {
        Self {
            generator: SelectorGenerator,
        }
    }

    /// Heal using a known previous fingerprint.
    pub fn heal(
        &self,
        failed_selector: &str,
        root: &DomNode,
        previous: Option<&DomFingerprint>,
    ) -> Vec<SelectorCandidate> {
        let mut paths = self.find_targets(root, failed_selector, previous);
        if paths.is_empty() {
            paths = self.best_fingerprint_matches(root, previous);
        }
        let mut out = Vec::new();
        for p in &paths {
            out.extend(strategies::attr_recovery(root, p, &self.generator));
            out.extend(strategies::structural_path(root, p, &self.generator));
            if let Some(fp) = previous {
                out.extend(strategies::text_proximity(root, fp));
            }
            out.extend(strategies::role_based(root, p, &self.generator));
            out.extend(strategies::sibling_proximity(root, p));
            out.extend(strategies::position_hint(root, p));
        }
        ranking::rank_candidates(root, out, previous)
    }

    fn find_targets(
        &self,
        root: &DomNode,
        sel: &str,
        fp: Option<&DomFingerprint>,
    ) -> Vec<Vec<usize>> {
        let mut m = self.generator.find(root, sel);
        if !m.is_empty() {
            return m;
        }
        if let Some(f) = fp {
            m = self.best_fingerprint_matches(root, Some(f));
        }
        m
    }

    fn best_fingerprint_matches(
        &self,
        root: &DomNode,
        fp: Option<&DomFingerprint>,
    ) -> Vec<Vec<usize>> {
        let Some(fp) = fp else { return vec![] };
        let mut all = Vec::new();
        root.walk(&mut all, vec![]);
        all.into_iter()
            .filter_map(|(p, _)| DomFingerprint::from_dom(root, &p).map(|f| (p, fp.similarity(&f))))
            .filter(|(_, s)| *s >= 0.55)
            .map(|(p, _)| p)
            .collect()
    }
}
