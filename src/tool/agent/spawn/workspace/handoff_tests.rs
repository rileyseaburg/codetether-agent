//! Handoff metadata preserves the original result identity and failure state.

use super::super::allocate;
use super::support::{fixture, head};
use crate::tool::ToolResult;
use serde_json::{Value, json};

#[tokio::test]
async fn handoff_keeps_agent_id_fields_and_success_flag() {
    let repo = fixture();
    let child = allocate(repo.path()).await.unwrap();
    for success in [true, false] {
        let body = json!({"agent_id": "child-agent", "success": success, "detail": "kept"});
        let original = if success {
            ToolResult::success(body.to_string())
        } else {
            ToolResult::error(body.to_string())
        };
        let result = child.attach(original);
        assert_eq!(result.success, success);
        let output: Value = serde_json::from_str(&result.output).unwrap();
        assert_eq!(output["agent_id"], "child-agent");
        assert_eq!(output["success"], success);
        assert_eq!(output["detail"], "kept");
        assert_eq!(output["isolation"]["mode"], "worktree");
        assert_eq!(output["isolation"]["auto_merge"], false);
        assert_eq!(
            output["isolation"]["integration"],
            "review_then_cherry_pick"
        );
        let checkout = &output["isolation"]["checkout"];
        assert_eq!(checkout["workspace"], json!(child.workspace));
        assert_eq!(checkout["worktree"], json!(child.worktree));
        assert_eq!(checkout["branch"], child.branch);
        assert_eq!(
            checkout["parent_workspace"],
            json!(repo.path().canonicalize().unwrap())
        );
        assert_eq!(checkout["base_commit"], head(repo.path()));
    }
}
