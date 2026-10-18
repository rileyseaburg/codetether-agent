use super::fixtures::*;
use crate::replay_oracle;
use codetether_provider_core::Role;

#[test]
fn report_over_empty_trace_is_empty() {
    let report = replay_oracle(&[], 4);
    assert!(report.demand.is_empty());
    assert_eq!(report.horizon, 4);
    assert_eq!(report.reference_count(), 0);
}
