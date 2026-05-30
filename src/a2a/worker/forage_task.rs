//! Forage task execution.

use anyhow::Result;

pub(super) async fn handle_forage_task(
    title: &str,
    prompt: &str,
    metadata: &serde_json::Map<String, serde_json::Value>,
    selected_model: Option<String>,
) -> Result<String> {
    let args = super::build_forage_args(prompt, title, metadata, selected_model);
    Ok(crate::forage::execute_with_summary(args)
        .await?
        .render_text())
}
