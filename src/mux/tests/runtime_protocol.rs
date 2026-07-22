use crate::mux::MuxRuntimeStatus;
use crate::mux::protocol::{ClientRequest, read_frame, write_frame};

#[tokio::test]
async fn runtime_status_round_trips_over_json_frame() {
    let request = ClientRequest::ReportRuntime {
        status: Some(MuxRuntimeStatus {
            session_id: "session-1234".into(),
            session_title: "Finish the office".into(),
            processing: true,
            message_count: 12,
            current_tool: Some("apply_patch".into()),
            needs_interaction: true,
            lagging: false,
            principal: crate::provenance::RuntimePrincipal {
                agent_name: "Morgan".into(),
                agent_identity_id: Some("ctagent_manager".into()),
                persona_id: Some("executive-vp".into()),
                spiffe_id: Some("spiffe://codetether.run/morgan".into()),
                provenance_id: Some("ctprov_session".into()),
            },
        }),
    };
    let (mut writer, mut reader) = tokio::io::duplex(1024);
    write_frame(&mut writer, &request).await.unwrap();
    let decoded: ClientRequest = read_frame(&mut reader).await.unwrap().unwrap();
    assert!(matches!(
        decoded,
        ClientRequest::ReportRuntime { status: Some(status) }
            if status.session_title == "Finish the office"
                && status.processing && status.message_count == 12
                && status.needs_interaction && !status.lagging
                && status.principal.agent_name == "Morgan"
                && status.principal.persona_id.as_deref() == Some("executive-vp")
    ));
}
