//! Original mux wire-roundtrip regression.
use crate::{AgentResponse, ServerResponse, read_frame, write_frame};
#[tokio::test]
async fn agent_output_carries_replay_and_completion_state() {
    let response = ServerResponse::Agent {
        response: AgentResponse::Output {
            task_id: "task-1234".into(),
            data: b"event\n".to_vec(),
            next_offset: 6,
            running: false,
            exit_code: Some(0),
        },
    };
    let (mut writer, mut reader) = tokio::io::duplex(1024);
    write_frame(&mut writer, &response).await.unwrap();

    let decoded: ServerResponse = read_frame(&mut reader).await.unwrap().unwrap();

    assert!(matches!(
        decoded,
        ServerResponse::Agent {
            response: AgentResponse::Output {
                exit_code: Some(0),
                ..
            }
        }
    ));
}
