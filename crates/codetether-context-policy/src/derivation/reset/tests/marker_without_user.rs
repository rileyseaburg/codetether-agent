//! Reset provenance suppresses the legacy fallback when no user remains.
use super::*;
#[test]
fn retains_marker_context_without_an_active_user() {
    let source = vec![
        message(Role::Assistant, "old"),
        markers::build_reset_summary_message("cached", ""),
        message(Role::Assistant, "work"),
    ];
    let ResetPreparation::Ready(context) = prepare(source, 3, 0, |_| 1) else {
        panic!("ready context expected")
    };
    assert_eq!(context.dropped_ranges, [(0, 1)]);
    assert_eq!(context.provenance, ["reset_marker"]);
    assert!(context.compressed);
}
