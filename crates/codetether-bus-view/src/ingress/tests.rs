use super::{Effects, State, maybe_queue};
use codetether_a2a_core::Part;
use codetether_bus_core::{AgentBus, BusEnvelope, BusMessage};
use codetether_worker_events::IncomingTask;
use std::collections::HashSet;

#[derive(Default)]
struct View {
    cursor: u64,
    agents: HashSet<String>,
    tasks: Vec<IncomingTask>,
}
impl State for View {
    fn cursor(&mut self) -> &mut u64 {
        &mut self.cursor
    }
    fn agents(&mut self) -> &mut HashSet<String> {
        &mut self.agents
    }
    fn is_registered(&self, agent: &str) -> bool {
        self.agents.contains(agent)
    }
    fn worker_id(&self) -> Option<&str> {
        None
    }
    fn session_id(&self) -> Option<&str> {
        Some("owner")
    }
}
impl Effects for View {
    fn observe_tasks(&mut self, _: &BusMessage) {}
    fn observe_tools(&mut self, _: &BusMessage) {}
    fn ingest_log(&mut self, _: &BusEnvelope) {}
    fn enqueue_task(&mut self, task: IncomingTask) {
        self.tasks.push(task);
    }
    fn owns_agent(&self, _: &str, _: &str) -> bool {
        false
    }
    fn knows_peer(&self, parent: &str, peer: &str) -> bool {
        parent == "owner" && peer == "peer"
    }
    fn push_assistant(&mut self, _: String) {}
    fn set_status(&mut self, _: String) {}
    fn scroll_to_bottom(&mut self) {}
}

#[test]
fn remote_reply_is_queued_for_its_owner_with_a_deduplication_marker() {
    let bus = AgentBus::new().into_arc();
    bus.handle("peer").send_to_agent(
        "owner",
        vec![Part::Text {
            text: "Let's discuss".into(),
        }],
    );
    let envelope = bus.recorder.recent(1, None).pop().unwrap();
    let mut view = View::default();
    maybe_queue(&mut view, &envelope);
    assert_eq!(view.tasks.len(), 1);
    assert!(
        view.tasks[0]
            .message
            .starts_with(&format!("[sub-agent-event:{}]", envelope.id))
    );
    assert!(view.tasks[0].message.contains("untrusted user input"));
    assert!(view.tasks[0].message.contains("Let's discuss"));
}

#[test]
fn unknown_sender_and_other_owner_do_not_resume_the_session() {
    let bus = AgentBus::new().into_arc();
    bus.handle("stranger").send_to_agent(
        "owner",
        vec![Part::Text {
            text: "hello".into(),
        }],
    );
    bus.handle("peer").send_to_agent(
        "other",
        vec![Part::Text {
            text: "hello".into(),
        }],
    );
    let mut view = View::default();
    for envelope in bus.recorder.recent(2, None) {
        maybe_queue(&mut view, &envelope);
    }
    assert!(view.tasks.is_empty());
}
