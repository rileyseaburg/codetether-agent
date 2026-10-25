//! Private state round trips with the Node-compatible envelope.
use super::super::{private_files, state};
use codetether_gemini_web_protocol::images::Continuation;

#[test]
fn node_compatible_state_is_private_and_not_overwritten() {
    let root = tempfile::tempdir().unwrap();
    let conversation = Continuation {
        conversation_id: "c_0123456789abcdef".into(),
        response_id: "r_0123456789abcdef".into(),
        choice_id: "rc_0123456789abcdef".into(),
    };
    let path = state::save(root.path(), &conversation).unwrap();
    assert_eq!(state::load(&path).unwrap(), conversation);
    assert_eq!(state::load(root.path()).unwrap(), conversation);
    let json: serde_json::Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(
        json["conversation"]["conversationId"],
        conversation.conversation_id
    );
    assert!(state::save(root.path(), &conversation).is_err());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            std::fs::metadata(path).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
}

#[test]
fn old_or_invalid_state_is_not_a_new_generation() {
    let root = tempfile::tempdir().unwrap();
    assert!(state::load(root.path()).is_err());
    private_files::write(&root.path().join("conversation.json"), br#"{"version":9}"#).unwrap();
    assert!(state::load(root.path()).is_err());
}
