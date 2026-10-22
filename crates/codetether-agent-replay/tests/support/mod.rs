use async_trait::async_trait;
use codetether_agent_core::ToolUse;
use codetether_agent_replay::Host;
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_tool_core::ToolResult;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct Disk {
    pub messages: Vec<Message>,
    pub pinned: bool,
    pub events: Vec<String>,
}
#[derive(Default)]
pub struct Session {
    pub messages: Vec<Message>,
    pub tools: Vec<ToolUse>,
    pub disk: Arc<Mutex<Disk>>,
    pub pinned: bool,
    pub fail_save: bool,
    pub fail_refresh: bool,
    pub deny: bool,
}
pub struct TestHost;
pub fn call(id: &str, name: &str) -> ContentPart {
    ContentPart::ToolCall {
        id: id.into(),
        name: name.into(),
        arguments: "{}".into(),
        thought_signature: None,
    }
}
pub fn session() -> Session {
    Session {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "preserve this input".into(),
            }],
        }],
        ..Default::default()
    }
}
#[async_trait]
impl Host for TestHost {
    type Session = Session;
    fn blocked(session: &Session, _: &str, _: &str) -> Option<ToolResult> {
        session.deny.then(|| ToolResult::error("denied"))
    }
    async fn persist_session(session: &mut Session) -> anyhow::Result<()> {
        if session.fail_save {
            anyhow::bail!("save failed");
        }
        let mut disk = session.disk.lock().unwrap();
        disk.events.push("save".into());
        disk.messages = session.messages.clone();
        Ok(())
    }
    async fn refresh_session(session: &mut Session) -> anyhow::Result<()> {
        if session.fail_refresh {
            anyhow::bail!("refresh failed");
        }
        let mut disk = session.disk.lock().unwrap();
        disk.events.push("refresh".into());
        session.messages = disk.messages.clone();
        session.pinned = disk.pinned;
        Ok(())
    }
    async fn execute_tool_for_session(
        &self,
        session: &Session,
        _: &str,
        name: &str,
        _: &str,
    ) -> ToolResult {
        let mut disk = session.disk.lock().unwrap();
        disk.events.push(format!("run:{name}:{}", session.pinned));
        if name == "context_pin" {
            disk.pinned = true;
        }
        ToolResult::success(format!("result:{name}"))
    }
    fn record_tool(session: &mut Session, usage: ToolUse) {
        session.tools.push(usage);
    }
    fn add_message(session: &mut Session, message: Message) {
        session.disk.lock().unwrap().events.push("record".into());
        session.messages.push(message);
    }
}
