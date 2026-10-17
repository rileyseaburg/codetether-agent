//! Regression cases for the parent stream module.

use super::{ResyncReason, parse_resync};

#[test]
fn parses_epoch_mismatch() {
    let r = parse_resync(r#"{"reason":"epoch_mismatch","head_seq":0,"epoch":"x"}"#).unwrap();
    assert_eq!(r.reason, ResyncReason::EpochMismatch);
    assert_eq!(r.epoch, "x");
}

#[test]
fn rejects_unknown_reason() {
    assert!(parse_resync(r#"{"reason":"nope","head_seq":1,"epoch":"x"}"#).is_err());
}
