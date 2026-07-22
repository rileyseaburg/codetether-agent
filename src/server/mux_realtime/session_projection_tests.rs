use super::project;
use crate::mux::MuxRuntimeStatus;
use crate::mux::control::MuxSessionSummary;
use crate::provenance::RuntimePrincipal;

#[test]
fn sessions_api_preserves_runtime_principal() {
    let session = MuxSessionSummary {
        name: "spotless-5".into(),
        address: "127.0.0.1:9876".into(),
        pid: 42,
        active_window: 0,
        windows: Vec::new(),
        reachable: true,
        runtime: Some(MuxRuntimeStatus {
            session_id: "internal-id".into(),
            session_title: "Fix customer scheduling".into(),
            processing: true,
            message_count: 8,
            current_tool: Some("apply_patch".into()),
            needs_interaction: false,
            lagging: false,
            principal: RuntimePrincipal {
                agent_name: "Morgan".into(),
                agent_identity_id: Some("ctagent_manager".into()),
                persona_id: Some("executive-vp".into()),
                spiffe_id: Some("spiffe://codetether.run/morgan".into()),
                provenance_id: Some("ctprov_session".into()),
            },
        }),
    };
    let value = project(session);
    assert_eq!(value["agent_name"], "Morgan");
    assert_eq!(value["persona_id"], "executive-vp");
    assert_eq!(value["runtime"]["current_tool"], "apply_patch");
    assert_eq!(
        value["runtime"]["principal"]["provenance_id"],
        "ctprov_session"
    );
    assert!(value["runtime"].get("session_id").is_none());
}
