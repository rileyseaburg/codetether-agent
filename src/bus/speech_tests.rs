//! Tests for the [`SpeechAct`](super::SpeechAct) agent language primitives.

use super::SpeechAct;

#[test]
fn roundtrip_act_strings() {
    for act in [
        SpeechAct::Request,
        SpeechAct::Inform,
        SpeechAct::Propose,
        SpeechAct::Accept,
        SpeechAct::Reject,
        SpeechAct::Claim,
        SpeechAct::Blocked,
        SpeechAct::Done,
        SpeechAct::Say,
    ] {
        assert_eq!(SpeechAct::from_str_lenient(act.as_str()), act);
    }
}

#[test]
fn lenient_aliases_and_unknown() {
    assert_eq!(SpeechAct::from_str_lenient("ASK"), SpeechAct::Request);
    assert_eq!(SpeechAct::from_str_lenient("agree"), SpeechAct::Accept);
    assert_eq!(SpeechAct::from_str_lenient("garbage"), SpeechAct::Say);
}

#[test]
fn reply_expectations() {
    assert!(SpeechAct::Request.expects_reply());
    assert!(!SpeechAct::Inform.expects_reply());
}
