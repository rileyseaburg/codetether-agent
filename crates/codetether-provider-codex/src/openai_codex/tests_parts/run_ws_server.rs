async fn run_ws_server(server_io: tokio::io::DuplexStream) {
    let callback = |request: &ServerRequest, response: ServerResponse| {
        assert_eq!(request.uri().path(), "/v1/responses");
        assert_eq!(
            request
                .headers()
                .get("Authorization")
                .and_then(|value| value.to_str().ok()),
            Some("Bearer test-token")
        );
        Ok(response)
    };
    let mut socket = accept_hdr_async(server_io, callback)
        .await
        .expect("server websocket handshake should succeed");
    let message = socket
        .next()
        .await
        .expect("server should receive message")
        .expect("server message should be valid");
    match message {
        WsMessage::Text(text) => {
            let event: Value = serde_json::from_str(&text).expect("server should parse JSON event");
            assert_eq!(
                event.get("type").and_then(Value::as_str),
                Some("response.create")
            );
        }
        other => panic!("expected text frame, got {other:?}"),
    }
    socket
        .send(WsMessage::Text(
            json!({ "type": "response.created" }).to_string().into(),
        ))
        .await
        .expect("server should send response");
}
