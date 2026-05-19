use serde_json::json;

use crate::tool::Tool;
use crate::tool::tetherscript::TetherScriptPluginTool;

#[tokio::test]
async fn executes_current_language_feature_sampler() {
    let tool = TetherScriptPluginTool::new();
    let result = tool
        .execute(json!({
            "path": "examples/tetherscript/language_features.tether",
            "hook": "summarize",
            "args": ["codetether"]
        }))
        .await
        .unwrap();

    assert!(result.success);
    assert_eq!(
        result.metadata.get("value"),
        Some(&json!({
            "ok": {
                "upper": "CODETETHER",
                "total_len": 21,
                "bytes_hex": "545321",
                "closure": 42
            }
        }))
    );
}
