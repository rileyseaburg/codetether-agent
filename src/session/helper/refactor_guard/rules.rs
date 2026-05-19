use super::types::GuardReport;

pub fn apply(report: &mut GuardReport) {
    super::budget::apply(report);
    super::displacement::apply(report);
}
