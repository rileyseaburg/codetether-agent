use super::{Route, steer};
use crate::provider::ContentPart;
use crate::session::{Session, helper::steering};
use crate::tool::agent::store::{self, AgentEntry};

#[tokio::test]
async fn running_child_accepts_same_turn_input() {
    let child = Session::new().await.unwrap();
    let id = child.id.clone();
    store::insert(AgentEntry {
        name: "active-child".into(),
        instructions: "test".into(),
        session: child,
        parent: None,
        owner_session_id: Some("active-owner".into()),
        depth: 0,
        model_id: None,
    });
    steering::open(&id);
    assert_eq!(
        steer("active-child", Some("active-owner"), "redirect now").await,
        Route::Steered
    );
    let mut session = store::get(&id).unwrap().session;
    assert_eq!(steering::drain_into(&mut session), 1);
    assert!(
        session
            .messages
            .last()
            .unwrap()
            .content
            .iter()
            .any(|part| matches!(part, ContentPart::Text { text } if text == "redirect now"))
    );
    steering::clear(&id);
    store::remove(&id);
}
