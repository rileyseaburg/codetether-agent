use chrono::Utc;

use super::super::store::{self, AgentEntry};
use crate::a2a::types::Part;
use crate::bus::{BusEnvelope, BusMessage};
use crate::session::Session;
use crate::tui::app::state::App;

#[tokio::test]
async fn owned_session_result_is_rendered_without_becoming_a_prompt() {
    let child = Session::new().await.expect("child session");
    let child_id = child.id.clone();
    let name = format!("render-{}", child.id);
    let parent = format!("parent-{}", child.id);
    store::insert(AgentEntry {
        name: name.clone(),
        instructions: "test".into(),
        session: child,
        parent: None,
        owner_session_id: Some(parent.clone()),
        depth: 0,
        model_id: Some("test/model".into()),
    });
    let envelope = BusEnvelope {
        id: "result-1".into(),
        topic: format!("agent.{parent}"),
        sender_id: name.clone(),
        correlation_id: None,
        timestamp: Utc::now(),
        message: BusMessage::AgentMessage {
            from: name.clone(),
            to: parent.clone(),
            parts: vec![Part::Text {
                text: "done\u{1b}".into(),
            }],
        },
    };
    let mut app = App::default();
    app.state.session_id = Some(parent);
    crate::tui::app::bus::inbox::maybe_queue(&mut app, &envelope);
    assert!(
        app.state
            .messages
            .iter()
            .any(|message| message.content == "done")
    );
    assert!(app.state.dequeue_worker_task().is_none());
    store::remove(&child_id);
}
