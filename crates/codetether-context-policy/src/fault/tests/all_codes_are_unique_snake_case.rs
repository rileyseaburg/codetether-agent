//! Fault contract regression.
use crate::fault::Fault;

#[test]
fn all_codes_are_unique_snake_case() {
    let faults = [
        Fault::NoMatch,
        Fault::Denied { reason: "x".into() },
        Fault::BackendError { reason: "y".into() },
        Fault::FlushMiss,
        Fault::PostCompactionBootstrap,
        Fault::PinnedInvariantMiss,
        Fault::DuplicateTool,
        Fault::Refetch,
    ];
    let codes: Vec<&'static str> = faults.iter().map(Fault::code).collect();
    let mut seen = codes.clone();
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(seen.len(), codes.len(), "reason codes must be unique");
    for c in &codes {
        assert!(!c.is_empty());
        assert_eq!(c.to_ascii_lowercase(), *c, "codes must be snake_case");
    }
}
