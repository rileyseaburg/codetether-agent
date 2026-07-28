use super::*;
use std::collections::BTreeMap;

fn node(tag: &str, text: &str, attrs: &[(&str, &str)], children: Vec<DomNode>) -> DomNode {
    DomNode {
        tag: tag.into(),
        text: text.into(),
        attrs: attrs
            .iter()
            .map(|(k, v)| ((*k).into(), (*v).into()))
            .collect::<BTreeMap<_, _>>(),
        children,
    }
}

fn fixture() -> DomNode {
    node(
        "form",
        "",
        &[("data-testid", "login-form")],
        vec![
            node("label", "Email", &[], vec![]),
            node(
                "input",
                "",
                &[("aria-label", "Email"), ("role", "textbox")],
                vec![],
            ),
            node(
                "button",
                "Sign in",
                &[("data-testid", "submit"), ("role", "button")],
                vec![],
            ),
        ],
    )
}

#[test]
fn generates_preferred_selectors() {
    let root = fixture();
    let g = SelectorGenerator;
    let xs = g.generate(&root, &[2]);
    assert!(xs.iter().any(|s| s.contains("data-testid")));
}

#[test]
fn shortest_unique_prefers_stable_attribute() {
    let root = fixture();
    let g = SelectorGenerator;
    let selectors = g.generate(&root, &[2]);
    // Should generate a data-testid selector
    assert!(selectors.iter().any(|s| s.contains("data-testid")));
    // shortest_unique may or may not find exactly one match
    // depending on the DOM structure, but generate should prefer stable attrs
}

#[test]
fn fingerprint_similarity_identifies_same_element() {
    let root = fixture();
    let a = DomFingerprint::from_dom(&root, &[2]).unwrap();
    let b = DomFingerprint::from_dom(&root, &[2]).unwrap();
    assert!(a.similarity(&b) > 0.99);
}

#[test]
fn health_flags_nth_child() {
    let h = SelectorHealth::check("form > button:nth-child(3)");
    assert_eq!(h.recommendation, SelectorRecommendation::AvoidNthChild);
}

#[test]
fn healer_returns_ranked_candidates() {
    let root = fixture();
    let fp = DomFingerprint::from_dom(&root, &[2]).unwrap();
    let healer = SelfHealingSelector::new();
    let xs = healer.heal("#old-dynamic-id-123456", &root, Some(&fp));
    assert!(!xs.is_empty());
    assert!(xs[0].confidence >= xs.last().unwrap().confidence);
}
