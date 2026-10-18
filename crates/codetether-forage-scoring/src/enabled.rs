//! Run the optional score hook through the existing Tokio runtime.
use std::path::Path;

use serde_json::json;

use super::{Host, ScoreInput, event::event, score_value::score_value};

const SCRIPT: &str = ".codetether/forage_score.tether";

pub(super) fn score_with_tetherscript<H: Host>(input: &ScoreInput<'_>) -> Option<f64> {
    if !Path::new(SCRIPT).exists() {
        return None;
    }
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async { run_script::<H>(input).await })
    })
}

async fn run_script<H: Host>(input: &ScoreInput<'_>) -> Option<f64> {
    let args = json!({ "path": SCRIPT, "hook": "score", "args": [event(input)] });
    if H::evaluate_tool_invocation("tetherscript_plugin", &args)
        .await
        .is_some()
    {
        return None;
    }
    let result = H::execute(args).await.ok()?;
    result.metadata.get("value").and_then(score_value)
}
