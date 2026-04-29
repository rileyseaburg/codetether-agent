use std::path::Path;

use serde_json::{Value, json};

use super::ScoreInput;
use crate::tool::Tool;
use crate::tool::tetherscript::TetherScriptPluginTool;

const SCRIPT: &str = ".codetether/forage_score.tether";

pub(super) fn score_with_tetherscript(input: &ScoreInput<'_>) -> Option<f64> {
    if !Path::new(SCRIPT).exists() {
        return None;
    }
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async { run_script(input).await })
    })
}

async fn run_script(input: &ScoreInput<'_>) -> Option<f64> {
    let result = TetherScriptPluginTool::new()
        .execute(json!({ "path": SCRIPT, "hook": "score", "args": [event(input)] }))
        .await
        .ok()?;
    result.metadata.get("value").and_then(score_value)
}

fn event(input: &ScoreInput<'_>) -> Value {
    json!({
        "base_score": input.base_score,
        "remaining": input.remaining,
        "moonshot_alignment": input.moonshot_alignment,
        "okr": { "id": input.okr.id, "title": input.okr.title },
        "key_result": { "id": input.kr.id, "title": input.kr.title }
    })
}

fn score_value(value: &Value) -> Option<f64> {
    match value {
        Value::Number(number) => number.as_f64(),
        Value::Object(map) => map.get("ok").and_then(Value::as_f64),
        _ => None,
    }
}
