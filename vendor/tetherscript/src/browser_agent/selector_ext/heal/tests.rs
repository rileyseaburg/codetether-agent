//! Self-healing selector tests.

use super::*;

fn props(tag: &str, id: &str, classes: &[&str], text: &str) -> ElementProps {
    ElementProps {
        tag: tag.into(),
        id: if id.is_empty() { None } else { Some(id.into()) },
        classes: classes.iter().map(|s| s.to_string()).collect(),
        text: if text.is_empty() {
            None
        } else {
            Some(text.into())
        },
        ..Default::default()
    }
}

#[test]
fn scores_exact_match_high() {
    let a = props("button", "save", &["primary"], "Save");
    assert!(element_similarity(&a, &a) >= 0.85);
}

#[test]
fn heals_to_best_candidate() {
    let original = props("button", "save", &["primary"], "Save");
    let bad = props("a", "cancel", &["link"], "Cancel");
    let good = props("button", "", &["primary"], "Save changes");
    assert_eq!(heal_selector(&original, &[bad, good], 0.45), Some(1));
}

#[test]
fn cache_roundtrip() {
    let mut cache = SelectorHealCache::new();
    let p = props("input", "email", &[], "");
    cache.remember("#email", p.clone());
    assert_eq!(cache.get("#email"), Some(&p));
}
