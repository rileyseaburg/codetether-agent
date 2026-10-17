use super::*;

#[test]
fn test_ephemeral_name_unique() {
    let a = AgentRegistry::ephemeral_name("sub");
    let b = AgentRegistry::ephemeral_name("sub");
    assert_ne!(a, b);
}
