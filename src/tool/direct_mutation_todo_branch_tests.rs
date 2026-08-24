//! Every todo mutation branch crosses the backend approval boundary.

use crate::approval::{test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, todo::TodoWriteTool};
use serde_json::json;

#[tokio::test]
async fn update_delete_and_clear_are_blocked_before_storage_access() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let root = tempfile::tempdir().expect("root");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let tool = TodoWriteTool::with_root(root.path().to_path_buf());
    let inputs = [
        json!({"action": "update", "id": "todo-1", "content": "changed"}),
        json!({"action": "delete", "id": "todo-1"}),
        json!({"action": "clear"}),
    ];
    for input in inputs {
        let result = tool.execute(input).await.expect("blocked");
        assert!(!result.success);
        assert!(result.metadata["approval_request_id"].is_string());
    }
}