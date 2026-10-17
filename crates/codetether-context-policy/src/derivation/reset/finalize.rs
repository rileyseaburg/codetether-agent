//! Merge reset-marker provenance and the newly summarized source range.
use super::*;
/// Attach original transcript ranges and prior reset provenance.
pub fn finalize(plan: ResetSummaryPlan, mut derived: DerivedContext) -> DerivedContext {
    let ResetSummaryPlan {
        prefix,
        base_index,
        mut dropped_ranges,
        mut provenance,
        ..
    } = plan;
    let prefix_len = prefix.len();
    dropped_ranges.push((base_index, base_index + prefix_len));
    if !provenance.is_empty() {
        provenance.extend(derived.provenance);
        derived.provenance = provenance;
    }
    derived.dropped_ranges = dropped_ranges;
    derived
}
