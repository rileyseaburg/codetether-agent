//! Parse one complete SSE event into an incoming worker task.

use super::IncomingTask;

pub(super) fn parse(event: &str) -> Option<IncomingTask> {
    let data = event
        .lines()
        .find_map(|line| line.strip_prefix("data:"))?
        .trim();
    if data.is_empty() || data == "[DONE]" {
        return None;
    }
    let task: serde_json::Value = serde_json::from_str(data).ok()?;
    Some(IncomingTask {
        task_id: text(&task, "task_id")
            .or_else(|| text(&task, "id"))
            .unwrap_or("unknown")
            .to_string(),
        message: text(&task, "message")
            .or_else(|| text(&task, "text"))
            .unwrap_or_default()
            .to_string(),
        from_agent: text(&task, "from_agent")
            .or_else(|| text(&task, "agent"))
            .map(str::to_string),
    })
}

fn text<'a>(value: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(serde_json::Value::as_str)
}
