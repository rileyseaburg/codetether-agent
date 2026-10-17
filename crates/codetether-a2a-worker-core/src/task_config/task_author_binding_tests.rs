use serde_json::json;

use super::{preserve_workspace, resume_session_id, verified};

#[test]
fn verified_protocol_can_resume_its_signed_session() {
    let metadata = json!({
        "protocol": "codetether.forgejo-author.v1",
        "server_author_binding_verified": true,
        "provenance_verified": true,
        "preserve_session_workspace": true,
        "resume_session_id": "author-session",
    });
    let metadata = metadata.as_object().unwrap();
    assert!(verified(metadata));
    let session_id = resume_session_id(metadata);
    assert_eq!(session_id.as_deref(), Some("author-session"));
    assert!(preserve_workspace(metadata, &session_id));
}

#[test]
fn client_boolean_cannot_authorize_protocol_resume() {
    let metadata = json!({
        "protocol": "codetether.forgejo-author.v1",
        "provenance_verified": true,
        "preserve_session_workspace": true,
        "resume_session_id": "victim-session",
    });
    let metadata = metadata.as_object().unwrap();
    assert!(!verified(metadata));
    assert_eq!(resume_session_id(metadata), None);
}

#[test]
fn ordinary_internal_resume_remains_compatible() {
    let metadata = json!({"resume_session_id": "ordinary-session"});
    let metadata = metadata.as_object().unwrap();
    assert_eq!(
        resume_session_id(metadata).as_deref(),
        Some("ordinary-session")
    );
}
