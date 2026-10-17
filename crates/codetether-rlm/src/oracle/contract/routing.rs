//! Structured and unstructured answer routing.
use super::*;
#[test]
fn validator_routing_snapshot() {
    let source = "async fn one() {}\nasync fn two() {}\nstruct Config {}";
    let snapshots: Vec<_> = super::cases::CASES
        .iter()
        .copied()
        .map(|(query, answer, kind)| {
            let result = capture(TraceValidator::new().validate(
                &analysis(query, answer),
                source,
                Some("main.rs"),
                Some("revision"),
                None,
            ));
            assert_eq!(result["kind"], kind);
            result
        })
        .collect();
    println!("ROUTING={}", serde_json::to_string(&snapshots).unwrap());
}
