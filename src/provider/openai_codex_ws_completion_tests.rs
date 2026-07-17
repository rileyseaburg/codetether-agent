#[tokio::test]
async fn completed_ws_releases_buffered_output() {
    let (client_io, server_io) = duplex(16 * 1024);
    let server = tokio::spawn(async move {
        let mut socket =
            accept_hdr_async(server_io, |_: &ServerRequest, response: ServerResponse| {
                Ok(response)
            })
            .await
            .unwrap();
        socket.next().await.unwrap().unwrap();
        for event in [
            json!({"type":"response.output_text.delta","delta":"complete"}),
            json!({"type":"response.completed","response":{"status":"completed"}}),
        ] {
            socket
                .send(WsMessage::Text(event.to_string().into()))
                .await
                .unwrap();
        }
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
        health.clone(),
        pool.clone(),
    )
    .collect::<Vec<_>>()
    .await;
    server.await.unwrap();
    assert!(matches!(&chunks[0], StreamChunk::Text(text) if text == "complete"));
    assert!(matches!(&chunks[1], StreamChunk::Done { .. }));
    assert!(!health.requires_http());
    assert!(pool.take().await.is_some());
}
