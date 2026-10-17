//! Fault contract regression.
use crate::fault::Fault;

#[test]
fn denied_carries_reason_string() {
    let f = Fault::Denied {
        reason: "policy rejected".into(),
    };
    assert!(f.is_denied());
    assert!(f.to_string().contains("policy rejected"));
    assert_eq!(f.code(), "denied");
}
