use crate::mux::protocol::{ClientRequest, read_frame, write_frame};

#[tokio::test]
async fn request_round_trips_over_json_frame() {
    let (mut writer, mut reader) = tokio::io::duplex(1024);
    write_frame(&mut writer, &ClientRequest::SelectWindow { id: 9 })
        .await
        .unwrap();

    let request: ClientRequest = read_frame(&mut reader).await.unwrap().unwrap();
    assert!(matches!(request, ClientRequest::SelectWindow { id: 9 }));
}
