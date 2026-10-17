//! Crash reporting load or create install id.
use super::*;
pub(super) fn load_or_create_install_id() -> Result<String> {
    let path = install_id_path();
    if let Ok(existing) = std::fs::read_to_string(&path) {
        let trimmed = existing.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create telemetry directory {}", parent.display()))?;
    }

    let new_id = Uuid::new_v4().to_string();
    std::fs::write(&path, format!("{new_id}\n"))
        .with_context(|| format!("write install id {}", path.display()))?;
    Ok(new_id)
}
