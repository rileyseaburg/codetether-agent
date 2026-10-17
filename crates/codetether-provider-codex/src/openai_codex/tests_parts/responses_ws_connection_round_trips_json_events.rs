#[tokio::test]
async fn responses_ws_connection_round_trips_json_events() {
    let (client_io, server_io) = duplex(16 * 1024);
    let server = tokio::spawn(run_ws_server(server_io));
    let request = OpenAiCodexProvider::build_responses_ws_request_with_base_url(
        "ws://localhost/v1/responses",
        "test-token",
    )
    .expect("client request should build");
    let (stream, _) = client_async(request, client_io)
        .await
        .expect("client websocket handshake should succeed");
    let mut connection = OpenAiRealtimeConnection::new(stream);
    connection
        .send_event(&json!({ "type": "response.create", "model": "gpt-5.4", "input": [] }))
        .await
        .expect("client should send event");
    let event = connection
        .recv_event()
        .await
        .expect("client should read event")
        .expect("client should receive session event");
    assert_eq!(
        event.get("type").and_then(Value::as_str),
        Some("response.created")
    );
    connection
        .close()
        .await
        .expect("client should close cleanly");
    server.await.expect("server task should finish");
}
