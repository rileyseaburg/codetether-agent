//! capability lease creation and validity.
use super::*;
#[test]
fn capability_lease_creation_and_validity() {
    let lease = CapabilityLease::new("p1", "Bash", "policy");
    assert!(lease.is_valid());
    assert_eq!(lease.persona_id, "p1");
    assert_eq!(lease.tool_name, "Bash");
}
