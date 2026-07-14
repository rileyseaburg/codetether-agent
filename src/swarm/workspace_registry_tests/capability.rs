use super::super::{Capability, with_defaults};
use serde_json::json;

#[tokio::test]
async fn verification_exposes_checks_but_rejects_mutation() {
    let root = tempfile::tempdir().unwrap();
    let registry = with_defaults(root.path(), Capability::Verification);
    for allowed in ["read", "lsp", "bash", "git"] {
        assert!(registry.contains(allowed), "missing {allowed}");
    }
    for denied in ["write", "edit", "apply_patch", "ralph", "agent"] {
        assert!(!registry.contains(denied), "unexpected {denied}");
    }
    let git = registry.get("git").unwrap();
    let result = git
        .execute(json!({"op":"commit", "message":"no"}))
        .await
        .unwrap();
    assert!(!result.success);
}

#[test]
fn mutating_registry_omits_unscoped_workflows() {
    let root = tempfile::tempdir().unwrap();
    let registry = with_defaults(root.path(), Capability::Mutating);
    for denied in ["ralph", "prd", "go", "agent", "swarm_execute", "undo"] {
        assert!(!registry.contains(denied), "unexpected {denied}");
    }
    assert!(registry.contains("apply_patch"));
}
