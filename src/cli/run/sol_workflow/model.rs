//! Model identifiers and worker-model validation for the Sol workflow.

use anyhow::{Result, ensure};

use super::super::RunArgs;

/// Fixed high-reasoning planner and LSP-review model.
pub(super) const SOL_PLANNER_MODEL: &str = "openai-codex/gpt-5.6-sol-fast:max";

/// Require a distinct explicit model for all source inspection and coding.
pub(super) fn worker_model(args: &RunArgs) -> Result<String> {
    let model = args
        .model
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow::anyhow!("--sol-planner requires --model for the coding worker"))?;
    ensure!(
        model != SOL_PLANNER_MODEL,
        "--model must name a coding worker, not the tool-free Sol planner"
    );
    Ok(model.to_string())
}

#[cfg(test)]
mod tests {
    use super::SOL_PLANNER_MODEL;

    #[test]
    fn pins_the_requested_sol_fast_max_model() {
        assert_eq!(SOL_PLANNER_MODEL, "openai-codex/gpt-5.6-sol-fast:max");
    }
}
