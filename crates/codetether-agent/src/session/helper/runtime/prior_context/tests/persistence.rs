use super::user;
use crate::session::Session;
use crate::session::helper::runtime::prior_context_allowed_for_session;

#[tokio::test]
async fn persistent_policy_survives_serialization_and_trimmed_history() {
    let mut session = Session::new().await.expect("session");
    session.add_human_message(user("never use session recall"));
    assert_eq!(session.metadata.prior_context_allowed, Some(false));

    let json = serde_json::to_string(&session).expect("serialize session");
    let mut restored: Session = serde_json::from_str(&json).expect("restore session");
    restored.messages.clear();
    restored.pages.clear();
    restored.add_human_message(user("continue with the repository docs"));
    assert!(!prior_context_allowed_for_session(&restored));

    restored.add_human_message(user("enable session recall"));
    assert!(prior_context_allowed_for_session(&restored));
    assert_eq!(restored.metadata.prior_context_allowed, Some(true));
}

#[tokio::test]
async fn repository_docs_policy_survives_serialization_and_followups() {
    let mut session = Session::new().await.expect("session");
    session.add_human_message(user("use the repository docs as the source of truth"));
    assert_eq!(session.metadata.prior_context_allowed, Some(false));

    let json = serde_json::to_string(&session).expect("serialize session");
    for followup in ["go", "continue", "fix it", "status?"] {
        let mut restored: Session = serde_json::from_str(&json).expect("restore session");
        restored.messages.clear();
        restored.pages.clear();
        restored.add_human_message(user(followup));
        assert!(!prior_context_allowed_for_session(&restored), "{followup}");
    }
}
