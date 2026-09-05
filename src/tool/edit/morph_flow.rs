//! Morph-backed edit previews retain their backend identity for callers.

use super::super::{ToolResult, morph_backend};
use super::args::EditArgs;
use super::diff;
use super::matcher::MatchPlan;
use super::metadata;
use super::morph;

pub async fn try_apply(args: &EditArgs<'_>, content: &str) -> Option<ToolResult> {
    if !should_use_morph(args) {
        return None;
    }
    let result = morph::apply(
        content,
        args.path,
        args.old_string,
        args.new_string,
        args.instruction,
        args.update,
    )
    .await;
    match result {
        Ok(Some(new_content)) => Some(preview(args.path, content, new_content)),
        Ok(None) => None,
        Err(error) => Some(error),
    }
}

fn should_use_morph(args: &EditArgs<'_>) -> bool {
    morph_backend::should_use_morph_backend()
        && (args.instruction.is_some() || args.update.is_some())
}

fn preview(path: &str, content: &str, new_content: String) -> ToolResult {
    let preview = diff::preview(content, &new_content);
    let plan = MatchPlan::find(content, content, false).expect("content matches itself");
    metadata::confirmation(path, content, &new_content, &plan, preview)
        .with_metadata("backend", serde_json::json!("morph"))
}
