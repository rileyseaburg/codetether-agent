//! API reports observed provider routing independently of a newly selected setting.
use super::support::{call, setup};
use serde_json::json;

#[tokio::test]
async fn verifier_model_api_identity_comes_from_harness_not_report_text() {
    let (app, _, observations) = setup();
    let mut attempt = observations.start();
    attempt.requested("provider/alias".into());
    attempt.resolved("actual-provider", "canonical-model");
    let (_, changed) = call(&app, "PUT", Some(json!({"model":"next/model"})), true).await;
    assert_eq!(changed["selected_model"], "next/model");
    assert_eq!(
        changed["latest_verification"]["resolved_provider"],
        "actual-provider"
    );
    assert_eq!(changed["latest_verification"]["state"], "running");
    attempt.finish(&Ok("I am a different model.\nVERDICT: PASS".into()));
    let (_, response) = call(&app, "GET", None, true).await;
    let identity = &response["latest_verification"];
    assert_eq!(identity["identity_source"], "harness_provider_resolution");
    assert_eq!(identity["requested_model"], "provider/alias");
    assert_eq!(identity["resolved_model"], "canonical-model");
    assert_eq!(identity["state"], "pass");
    assert!(identity["finished_at"].is_string());
    assert_eq!(attempt.identity(), "actual-provider/canonical-model");
}
