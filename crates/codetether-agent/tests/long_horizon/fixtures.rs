use codetether_agent::session::Session;
use codetether_context_pin::PinSource;
use codetether_context_policy::pages::PageKind;
use codetether_goal_core::{GoalRuntimeUpdate, GoalSourceKind, TaskEvent, TaskLog};
use codetether_provider_core::{ContentPart, Message, Role};
use serde_json::json;

pub const ORIGINAL: &str = "Keep all evidence 🦀.\nUse red.\nNever deploy.\nPreserve API.";
pub fn text(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
pub fn edit_input() -> serde_json::Value {
    json!({"file_path":"target","old_string":"old","new_string":"new"})
}
pub fn call(id: &str, name: &str, input: serde_json::Value) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: id.into(),
            name: name.into(),
            arguments: input.to_string(),
            thought_signature: None,
        }],
    }
}
pub fn reply(id: &str, success: bool, output: &str) -> Message {
    Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: id.into(),
            content: format!(
                "Tool call feedback\n- status: {}\n\nOutput:\n{output}",
                if success { "success" } else { "failure" }
            ),
        }],
    }
}
pub async fn session() -> anyhow::Result<Session> {
    let mut session = Session::new().await?;
    session.metadata.history_sink = None;
    session.metadata.subcall_provider = None;
    session.add_message(text(Role::User, ORIGINAL));
    session.pages[0] = PageKind::Constraint;
    session.add_message(text(Role::User, "Use blue."));
    for index in 0..3 {
        let id = format!("failed-{index}");
        session.add_message(call(&id, "edit", edit_input()));
        session.add_message(reply(&id, false, "target not found"));
    }
    session.add_message(call("pending", "read", json!({"path":"target"})));
    session.add_message(text(
        Role::Assistant,
        "Uncertain why the target differs. Next: inspect the source before retrying.",
    ));
    for index in 0..1_100 {
        session.add_message(text(Role::User, &format!("Archived observation {index}")));
    }
    Ok(session)
}
pub fn journal(log: &TaskLog, session: &Session) -> anyhow::Result<()> {
    log.append_blocking(&TaskEvent::GoalSet {
        at: chrono::Utc::now(),
        goal_id: "evaluation".into(),
        objective: "Finish the source-backed repair".into(),
        success_criteria: vec!["Verified repair".into()],
        forbidden: vec!["Never deploy".into()],
        source_session_id: session.id.clone(),
        source_turn_id: "0".into(),
        source_text_hash: session.constraint_source(0).unwrap().text_hash,
        source_kind: GoalSourceKind::UserProvided,
        confidence: 1.0,
    })?;
    Ok(())
}
pub fn charge(log: &TaskLog, tokens: i64) -> anyhow::Result<()> {
    log.append_blocking(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: chrono::Utc::now(),
        goal_id: "evaluation".into(),
        objective: None,
        status: None,
        token_budget: None,
        token_delta: tokens,
        elapsed_seconds: 1,
        continuation_delta: 0,
    }))
}
pub struct FileJournal;
impl codetether_goal_runtime::Journal for FileJournal {
    fn for_session(path: &str) -> anyhow::Result<TaskLog> {
        Ok(TaskLog::at(path))
    }
}

pub struct LocationGuard(pub String);
impl Drop for LocationGuard {
    fn drop(&mut self) {
        codetether_session_location::forget(&self.0);
    }
}
