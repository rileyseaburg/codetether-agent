//! Opt-in migration proof against a protected copy of the reported session.
use super::super::{load, save};
#[tokio::test]
#[ignore = "requires the locally preserved incident fixture"]
async fn migrate_incident_copy_without_touching_original() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = root.join(
        "artifacts/session-storage-84ee5af3/original/84ee5af3-841d-4741-81b3-7549c2420b31.json",
    );
    let original = std::fs::read(&source).unwrap();
    let value: serde_json::Value = serde_json::from_slice(&original).unwrap();
    let count = value["messages"].as_array().unwrap().len();
    let proof = root
        .join("artifacts/session-store")
        .join(format!("incident-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&proof).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&proof, std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let path = proof.join("84ee5af3-841d-4741-81b3-7549c2420b31.json");
    std::fs::write(&path, &original).unwrap();
    let mut loaded = load(&path, 1000).await.unwrap().session;
    assert_eq!(loaded.message_count(), count);
    assert_eq!(loaded.messages.len(), 1000);
    assert_eq!(loaded.id, value["id"].as_str().unwrap());
    loaded.add_message(super::message("isolated storage regression sentinel"));
    save(&loaded, &path).await.unwrap();
    let exported = load(&path, usize::MAX).await.unwrap().session;
    assert_eq!(exported.messages.len(), count + 1);
    assert_eq!(
        serde_json::to_value(&exported.messages[..count]).unwrap(),
        value["messages"]
    );
    assert_eq!(std::fs::read(&source).unwrap(), original);
    let report = serde_json::json!({"proof_level":"focused CI-like", "session_id":loaded.id,
        "original_messages":count,"retained_messages":loaded.messages.len(),"persisted_messages":exported.messages.len(),
        "original_unchanged":true,"database":proof.join("sessions.sqlite3")});
    let evidence = proof.join("result.json");
    std::fs::write(&evidence, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    eprintln!("incident proof: {}", evidence.display());
}
