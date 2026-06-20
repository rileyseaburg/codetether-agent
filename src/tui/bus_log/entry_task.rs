//! Entry builders for task, artifact, and shared-result messages.

use ratatui::style::Color;

use crate::{a2a::types::Artifact, bus::BusMessage};

use super::{entry_parts::EntryParts, truncate::truncate};

pub(super) fn entry_parts(message: &BusMessage) -> EntryParts {
    if let BusMessage::TaskUpdate {
        task_id,
        state,
        message,
    } = message
    {
        let text = message.as_deref().unwrap_or("");
        return EntryParts::new(
            "TASK",
            format!("{task_id} → {state:?} {}", truncate(text, 50)),
            format!("Task: {task_id}\nState: {state:?}\nMessage: {text}"),
            Color::Yellow,
        );
    }
    if let BusMessage::ArtifactUpdate { task_id, artifact } = message {
        return artifact_entry(task_id, artifact);
    }
    if let BusMessage::SharedResult { key, tags, .. } = message {
        return EntryParts::new(
            "RESULT",
            format!("key={key} tags=[{}]", tags.join(",")),
            format!("Key: {key}\nTags: {}", tags.join(", ")),
            Color::Blue,
        );
    }
    unreachable!("task entry builder received a non-task bus message");
}

fn artifact_entry(task_id: &str, artifact: &Artifact) -> EntryParts {
    EntryParts::new(
        "ARTIFACT",
        format!("task={task_id} parts={}", artifact.parts.len()),
        format!(
            "Task: {task_id}\nArtifact: {}\nParts: {}",
            artifact.name.as_deref().unwrap_or("(unnamed)"),
            artifact.parts.len()
        ),
        Color::Magenta,
    )
}
