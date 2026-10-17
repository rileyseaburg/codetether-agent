//! An existing marker can bring the retained tail below threshold.
use super::*;
#[test]
fn reuses_the_latest_marker_before_summarization() {
    let source = vec![
        message(Role::Assistant, "old"),
        message(Role::Assistant, "old"),
        markers::build_reset_summary_message("cached", ""),
        message(Role::User, "task"),
    ];
    let ResetPreparation::Ready(context) = prepare(source, 4, 2, |messages| messages.len()) else {
        panic!("ready context expected")
    };
    assert_eq!(context.messages.len(), 2);
    assert_eq!(context.dropped_ranges, [(0, 2)]);
    assert_eq!(context.provenance, ["reset_marker"]);
    assert!(context.compressed);
}
