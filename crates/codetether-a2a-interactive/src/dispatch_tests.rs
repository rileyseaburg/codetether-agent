use super::*;
use std::collections::VecDeque;

#[derive(Default)]
struct View {
    busy: bool,
    tasks: VecDeque<IncomingTask>,
    dispatched: Vec<String>,
}
impl Presentation for View {
    fn push_system(&mut self, _: String) {}
    fn set_status(&mut self, _: String) {}
    fn scroll_to_bottom(&mut self) {}
}
impl Dispatch for View {
    fn processing(&self) -> bool {
        self.busy
    }
    fn dequeue_task(&mut self) -> Option<IncomingTask> {
        self.tasks.pop_front()
    }
    fn task_delivered(&self, task: &IncomingTask) -> bool {
        task.task_id == "seen"
    }
    fn set_active_task(&mut self, _: IncomingTask) {}
    async fn dispatch_prompt(&mut self, prompt: &str) {
        self.dispatched.push(prompt.into());
    }
}

#[tokio::test]
async fn queued_reply_waits_for_idle_and_skips_already_consumed_events() {
    let mut view = View {
        busy: true,
        ..View::default()
    };
    for id in ["seen", "fresh"] {
        view.tasks.push_back(IncomingTask {
            task_id: id.into(),
            message: id.into(),
            from_agent: Some("peer".into()),
        });
    }
    trigger_next(&mut view).await;
    assert_eq!(view.tasks.len(), 2);
    assert!(view.dispatched.is_empty());
    view.busy = false;
    trigger_next(&mut view).await;
    assert_eq!(view.dispatched, ["fresh"]);
    assert!(view.tasks.is_empty());
}

impl codetether_tui_tick::background::Host for View {
    type Notice = ();
    fn models(&mut self) {}
    fn bus(&mut self) {}
    fn queue_workers(&mut self) {}
    async fn events(&mut self) {}
    fn next_notice(&mut self) -> Option<()> {
        None
    }
    async fn before_notice(&mut self) {}
    async fn notice(&mut self, _: ()) {}
}

#[tokio::test]
async fn background_drain_preserves_inbound_for_execution_exactly_once() {
    let mut view = View::default();
    view.tasks.push_back(IncomingTask {
        task_id: "request".into(),
        message: "Please answer this peer".into(),
        from_agent: Some("peer".into()),
    });
    // The reactor drains background updates before triggering the inbox.
    codetether_tui_tick::background::drain(&mut view).await;
    assert_eq!(view.tasks.len(), 1);
    trigger_next(&mut view).await;
    assert_eq!(view.dispatched, ["Please answer this peer"]);
    codetether_tui_tick::background::drain(&mut view).await;
    trigger_next(&mut view).await;
    assert_eq!(view.dispatched.len(), 1);
}

#[tokio::test]
async fn background_drain_retains_inbound_while_model_is_busy() {
    let mut view = View {
        busy: true,
        ..View::default()
    };
    view.tasks.push_back(IncomingTask {
        task_id: "request".into(),
        message: "Reply when idle".into(),
        from_agent: Some("peer".into()),
    });
    codetether_tui_tick::background::drain(&mut view).await;
    trigger_next(&mut view).await;
    assert_eq!(view.tasks.len(), 1);
    assert!(view.dispatched.is_empty());
    view.busy = false;
    codetether_tui_tick::background::drain(&mut view).await;
    trigger_next(&mut view).await;
    assert_eq!(view.dispatched, ["Reply when idle"]);
}
