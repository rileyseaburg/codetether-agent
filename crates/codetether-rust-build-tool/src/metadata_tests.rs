//! Native enrichment must not become command options or execution authority.
use super::*;
use codetether_provenance::ExecutionProvenance;
use codetether_runtime_input::enrich_tool_input_with_runtime_context;
use serde_json::json;
use std::path::Path;

#[tokio::test]
async fn runtime_enriched_plan_matches_plain_plan() {
    let provenance: ExecutionProvenance = serde_json::from_value(json!({
        "provenance_id": "fixture", "task_id": "task", "run_id": "run",
        "attempt_id": "attempt", "issued_at": "2026-01-01T00:00:00Z",
        "identity": {"agent_name": "agent", "origin": "swarm", "worker_id": "worker",
            "tenant_id": "tenant", "agent_identity_id": "fixture-agent", "key_id": "key"}
    }))
    .unwrap();
    for action in ["build", "run", "test", "check", "status", "doctor", "stop"] {
        let base = json!({"action": action, "cwd": env!("CARGO_MANIFEST_DIR")});
        let plain = RustBuildPlanTool.execute(base.clone()).await.unwrap();
        assert!(plain.success, "{}", plain.output);
        let mut enriched = enrich_tool_input_with_runtime_context(
            &base,
            Path::new(env!("CARGO_MANIFEST_DIR")),
            Some("fixture/model"),
            "fixture-session",
            "fixture-agent",
            Some(&provenance),
        );
        enriched["__ct_lease_owner"] = json!("fixture-turn");
        enriched["__ct_prior_context_allowed"] = json!(false);
        enriched["__ct_signature"] = json!("fixture-signature");
        assert_eq!(enriched["__ct_agent_identity_id"], "fixture-agent");
        let result = RustBuildPlanTool.execute(enriched).await.unwrap();
        assert!(result.success, "{}", result.output);
        assert_eq!(
            result.output, plain.output,
            "metadata must not change the plan"
        );
        assert!(!result.output.contains("fixture-agent"));
        let output: Value = serde_json::from_str(&result.output).unwrap();
        assert_eq!(output["executed"], false);
    }
}

#[tokio::test]
async fn runtime_metadata_does_not_admit_unknown_options_or_grants() {
    for key in [
        "approval_id",
        "justification",
        "sandbox_permissions",
        "cmd",
        "pakcage",
        "__ct_approval_id",
        "__ct_unknown",
    ] {
        let mut input = json!({"action": "build", "cwd": env!("CARGO_MANIFEST_DIR"),
            "__ct_agent_identity_id": "fixture-agent"});
        input[key] = json!("untrusted");
        let result = RustBuildPlanTool.execute(input).await.unwrap();
        assert!(!result.success, "accepted {key}");
        assert!(
            result.output.contains(&format!("unsupported field: {key}")),
            "{}",
            result.output
        );
    }
}
