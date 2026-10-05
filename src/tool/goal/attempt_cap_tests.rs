//! Infrastructure failures are audited without consuming or resetting rejection counts.
use super::rejections_since_reset;
use crate::tool::goal::verdict_log::VerdictRecord;

#[test]
fn unavailable_attempts_do_not_consume_or_erase_real_rejections() {
    let fail = VerdictRecord::new("g", "complete", false, "m", "VERDICT: FAIL");
    let mut unavailable = fail.clone();
    unavailable.unavailable = true;
    let log = [fail.clone(), unavailable, fail];
    assert_eq!(rejections_since_reset(&log, "g"), 2);
}

#[test]
fn older_records_without_unavailable_field_remain_readable() {
    let record = VerdictRecord::new("g", "complete", false, "m", "VERDICT: FAIL");
    let mut value = serde_json::to_value(record).unwrap();
    value.as_object_mut().unwrap().remove("unavailable");
    let old: VerdictRecord = serde_json::from_value(value).unwrap();
    assert!(!old.unavailable);
    assert_eq!(rejections_since_reset(&[old], "g"), 1);
}
