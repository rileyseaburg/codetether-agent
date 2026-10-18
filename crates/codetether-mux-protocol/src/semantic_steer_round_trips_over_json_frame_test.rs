//! Original mux wire-roundtrip regression.
use crate::{ClientRequest, ProgramRequest, read_frame, write_frame};
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
