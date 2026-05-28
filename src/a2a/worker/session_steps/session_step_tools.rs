use std::{path::Path, sync::Arc};

use crate::{
    provider::{ContentPart, Message, Role},
    session::Session,
};

mod tool_run;

pub(super) type ToolCall = (String, String, serde_json::Value);

pub(super) fn collect_tool_calls(parts: &[ContentPart]) -> Vec<ToolCall> {
    parts
        .iter()
        .filter_map(|part| match part {
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => Some((
                id.clone(),
                name.clone(),
                serde_json::from_str(arguments).unwrap_or(serde_json::json!({})),
            )),
            _ => None,
        })
        .collect()
}

pub(super) async fn execute_tool_call(
    session: &mut Session,
    registry: &crate::tool::ToolRegistry,
    auto_approve: super::super::AutoApprove,
    workspace_dir: &Path,
    model: &str,
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
    (tool_id, tool_name, tool_input): ToolCall,
) {
    if let Some(cb) = cb {
        cb(format!("[tool:start:{}]", tool_name));
    }
    if !super::super::is_tool_allowed(&tool_name, auto_approve) {
        add_tool_result(
            session,
            tool_id,
            format!(
                "Tool '{}' requires approval but auto-approve policy is {:?}",
                tool_name, auto_approve
            ),
        );
        return;
    }
    let input = super::super::enrich_tool_input_with_runtime_context(
        &tool_input,
        workspace_dir,
        Some(model),
        &session.id,
        &session.agent,
        session.metadata.provenance.as_ref(),
    );
    let output = tool_run::run_tool(registry, &tool_name, input, cb).await;
    add_tool_result(session, tool_id, output);
}

fn add_tool_result(session: &mut Session, tool_call_id: String, content: String) {
    session.add_message(Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id,
            content,
        }],
    });
}

pub(super) fn append_text_output(
    parts: &[ContentPart],
    out: &mut String,
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) {
    for part in parts {
        if let ContentPart::Text { text } = part
            && !text.is_empty()
        {
            out.push_str(text);
            out.push('\n');
            if let Some(cb) = cb {
                cb(text.clone());
            }
        }
    }
}
