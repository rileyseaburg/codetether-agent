#[tokio::test]
async fn completed_connection_handles_two_sequential_responses() {
    let (client_io, server_io) = duplex(16 * 1024);
    let server = tokio::spawn(async move {
        let mut socket =
            accept_hdr_async(server_io, |_: &ServerRequest, response: ServerResponse| {
                Ok(response)
            })
            .await
            .unwrap();
        for text in ["first", "second"] {
            socket.next().await.unwrap().unwrap();
            for event in [
                json!({"type":"response.output_text.delta","delta":text}),
                json!({"type":"response.completed","response":{"id":format!("resp-{text}"),"status":"completed"}}),
            ] {
                socket
                    .send(WsMessage::Text(event.to_string().into()))
                    .await
                    .unwrap();
            }
        }
    });
    let request = OpenAiCodexProvider::build_responses_ws_request_with_base_url(
        "ws://localhost/v1/responses",
        "test-token",
    )
    .unwrap();
    let (socket, _) = client_async(request, client_io).await.unwrap();
    let pool: WsPool<_> = WsPool::default();
    pool.put("session-a".to_string(), OpenAiRealtimeConnection::new(socket))
        .await;

    for expected in ["first", "second"] {
        assert!(pool.take("session-b").await.is_none());
        let connection = pool.take("session-a").await.expect("recycled connection");
        let chunks = ws_stream::drive(
            connection,
            json!({"type":"response.create"}),
            "session-a".to_string(),
            TransportHealth::default(),
            TurnStateStore::default(),
            pool.clone(),
        )
        .collect::<Vec<_>>()
        .await;
        assert!(matches!(&chunks[0], StreamChunk::Text(text) if text == expected));
    }
    server.await.unwrap();
    assert!(pool.take("session-a").await.is_some());
}
