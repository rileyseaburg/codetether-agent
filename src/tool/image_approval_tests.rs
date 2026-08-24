use super::ImageTool;
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::Tool;
use serde_json::json;

#[tokio::test]
async fn local_image_claims_exact_approval() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let _network = crate::tool::network_access::test_env::Network::set("0");
    let image = data.path().join("fixture.png");
    std::fs::write(&image, b"not-decoded-by-loader").expect("image fixture");
    let mut args = json!({
        "path": image,
        "__ct_session_id": "image-test",
        "__ct_parent_workspace": data.path(),
    });
    crate::tool::network_access::bind_trusted(&mut args, false);
    let blocked = crate::runtime_policy::evaluate_tool_invocation("image", &args)
        .await
        .expect("approval required");
    let request_id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(request_id, "test", "local image")
        .expect("approve");
    args["approval_id"] = json!(request_id);

    let result = ImageTool.execute(args.clone()).await.expect("image result");
    assert!(result.success, "{}", result.output);
    let replay = ImageTool.execute(args).await.expect("replay result");
    assert!(!replay.success);
    assert!(replay.output.contains("approval"));
}