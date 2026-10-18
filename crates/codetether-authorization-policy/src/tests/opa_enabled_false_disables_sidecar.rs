//! Existing authorization regression case.
use super::*;

#[test]
fn opa_enabled_false_disables_sidecar() {
    assert!(sidecar_disabled(false, Some("false")));
    assert!(sidecar_disabled(false, Some("FALSE")));
    assert!(!sidecar_disabled(false, None));
}
