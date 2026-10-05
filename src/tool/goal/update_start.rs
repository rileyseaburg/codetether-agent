//! Resolve the production verifier before entering the guarded update workflow.
use super::Args;
use crate::tool::{ToolResult, goal::verify::LlmVerifier};
use anyhow::Result;

pub(in crate::tool::goal) async fn run(args: Args) -> Result<ToolResult> {
    let verifier = LlmVerifier {
        worker_model: args.current_model.clone(),
        workspace: args.workspace.clone().map_or_else(
            || std::env::current_dir().unwrap_or_else(|_| ".".into()),
            Into::into,
        ),
    };
    super::run_with(args, &verifier).await
}
