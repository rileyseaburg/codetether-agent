#[tokio::test]
async fn interrupted_ws_surfaces_failure_without_disabling_ws() {
    let (client_io, server_io) = duplex(16 * 1024);
    let server = tokio::spawn(async move {
        let mut socket =
            accept_hdr_async(server_io, |_: &ServerRequest, response: ServerResponse| {
                Ok(response)
            })
            .await
            .unwrap();
        socket.next().await.unwrap().unwrap();
        socket
            .send(WsMessage::Text(
                json!({"type":"response.output_text.delta","delta":"secret partial"})
                    .to_string()
                    .into(),
            ))
            .await
            .unwrap();
        socket.close(None).await.unwrap();
    });
    let request = OpenAiCodexProvider::build_responses_ws_request_with_base_url(
        "ws://localhost/v1/responses",
        "test-token",
    )
    .unwrap();
    let (socket, _) = client_async(request, client_io).await.unwrap();
    let health = TransportHealth::default();
    let pool: WsPool<_> = WsPool::default();
    let chunks = ws_stream::drive(
        OpenAiRealtimeConnection::new(socket),
        json!({"type":"response.create"}),
        "session-a".to_string(),
        health.clone(),
        TurnStateStore::default(),
        pool.clone(),
    )
    .collect::<Vec<_>>()
    .await;
    server.await.unwrap();
    assert!(matches!(&chunks[0], StreamChunk::Text(text) if text == "secret partial"));
    assert!(matches!(&chunks[1], StreamChunk::Error(message) if message.contains("response.completed")));
    assert!(!health.requires_http("session-a"));
    assert!(!health.requires_http("session-b"));
    assert!(pool.take("session-a").await.is_none());
}
