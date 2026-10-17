//! Summary plans preserve the task tail and original source coverage.
use super::*;
#[test]
fn summarizes_only_the_prefix_and_records_coverage() {
    let mut source = vec![
        message(Role::Assistant, "old"),
        message(Role::User, "implement feature"),
    ];
    source.extend((0..8).map(|_| message(Role::Assistant, "work")));
    let ResetPreparation::Summarize(plan) = prepare(source, 10, 0, |_| 1) else {
        panic!("summary plan expected")
    };
    assert_eq!(plan.prefix.len(), 1);
    assert_eq!(plan.tail.len(), 9);
    let rebuilt = rebuild("summary", "", &plan.tail, plan.origin_len);
    let context = finalize(plan, rebuilt);
    assert_eq!(context.messages.len(), 10);
    assert_eq!(context.dropped_ranges, [(0, 1)]);
    assert_eq!(context.provenance, ["reset", "rlm_summary"]);
    assert!(context.compressed);
}
