//! Canonical task-name generation and validation.

pub(super) fn generated() -> String {
    format!(
        "agent_{}",
        &uuid::Uuid::new_v4().simple().to_string()[..8]
    )
}

pub(super) fn resolve(requested: Option<String>) -> anyhow::Result<String> {
    let name = requested.unwrap_or_else(generated);
    let valid = name != "root"
        && !name.is_empty()
        && name
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_');
    anyhow::ensure!(
        valid,
        "task_name must use only lowercase letters, digits, and underscores"
    );
    Ok(name)
}

#[cfg(test)]
#[path = "spawn_name_tests.rs"]
mod tests;
