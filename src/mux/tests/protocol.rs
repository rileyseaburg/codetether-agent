use crate::mux::lease::CoordinationRequest;
use crate::mux::protocol::{ClientRequest, ProgramRequest, read_frame, write_frame};

#[tokio::test]
async fn request_round_trips_over_json_frame() {
    let (mut writer, mut reader) = tokio::io::duplex(1024);
    write_frame(&mut writer, &ClientRequest::SelectWindow { id: 9 })
        .await
        .unwrap();

    let request: ClientRequest = read_frame(&mut reader).await.unwrap().unwrap();
    assert!(matches!(request, ClientRequest::SelectWindow { id: 9 }));
}

#[tokio::test]
async fn semantic_steer_round_trips_over_json_frame() {
    let request = ClientRequest::Program {
        request: ProgramRequest::Steer {
            text: "finish validation".into(),
        },
    };
    let (mut writer, mut reader) = tokio::io::duplex(1024);
    write_frame(&mut writer, &request).await.unwrap();
    let decoded: ClientRequest = read_frame(&mut reader).await.unwrap().unwrap();
    assert!(matches!(
        decoded,
        ClientRequest::Program {
            request: ProgramRequest::Steer { .. }
        }
    ));
}

#[tokio::test]
async fn coordination_request_round_trips_over_json_frame() {
    let request = ClientRequest::Coordinate {
        request: CoordinationRequest::Acquire {
            owner: "turn-1".into(),
            agent: "agent-a".into(),
            workspace: "/workspace".into(),
            paths: vec!["src/lib.rs".into()],
            wait_ms: 60_000,
        },
    };
    let (mut writer, mut reader) = tokio::io::duplex(1024);
    write_frame(&mut writer, &request).await.unwrap();
    let decoded: ClientRequest = read_frame(&mut reader).await.unwrap().unwrap();
    assert!(matches!(decoded, ClientRequest::Coordinate { .. }));
}
