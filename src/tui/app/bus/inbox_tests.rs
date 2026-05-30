use chrono::Utc;

use super::inbox::maybe_queue;
use crate::a2a::types::Part;
use crate::bus::{BusEnvelope, BusMessage};
use crate::tui::app::state::App;

fn envelope(to: &str) -> BusEnvelope {
    BusEnvelope {
        id: "env-1".to_string(),
        topic: format!("agent.{to}"),
        sender_id: "tester".to_string(),
        correlation_id: Some("corr-1".to_string()),
        timestamp: Utc::now(),
        message: BusMessage::AgentMessage {
            from: "tester".to_string(),
            to: to.to_string(),
            parts: vec![Part::Text {
                text: "hello bus".to_string(),
            }],
        },
    }
}

#[test]
fn direct_message_to_tui_queues_untrusted_task() {
    let mut app = App::default();
    maybe_queue(&mut app, &envelope("tui"));

    let task = app.state.dequeue_worker_task().expect("queued task");
    assert_eq!(task.task_id, "corr-1");
    assert_eq!(task.from_agent.as_deref(), Some("tester"));
    assert!(task.message.contains("untrusted user input"));
    assert!(task.message.contains("hello bus"));
}

#[test]
fn message_to_other_agent_is_ignored() {
    let mut app = App::default();
    maybe_queue(&mut app, &envelope("other"));

    assert!(app.state.dequeue_worker_task().is_none());
}
