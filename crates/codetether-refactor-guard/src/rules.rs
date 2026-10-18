//! Apply built-in guard rules in their established order.
use super::types::GuardReport;

pub(super) fn apply(report: &mut GuardReport) {
    super::budget::apply(report);
    super::displacement::apply(report);
}
