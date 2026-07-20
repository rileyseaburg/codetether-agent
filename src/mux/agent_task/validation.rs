//! Input bounds for authenticated mux agent tasks.

use anyhow::{Result, ensure};

pub(super) fn request(
    task_id: &str,
    prompt: &str,
    session_id: Option<&str>,
    max_steps: usize,
) -> Result<()> {
    ensure!(valid_id(task_id), "invalid agent task id");
    ensure!(!prompt.trim().is_empty(), "agent prompt is empty");
    ensure!(prompt.len() <= 64 * 1024, "agent prompt exceeds 64 KiB");
    ensure!(
        (1..=250).contains(&max_steps),
        "max_steps must be between 1 and 250"
    );
    ensure!(
        session_id.is_none_or(valid_id),
        "invalid durable session id"
    );
    Ok(())
}

fn valid_id(value: &str) -> bool {
    (8..=128).contains(&value.len())
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
}
