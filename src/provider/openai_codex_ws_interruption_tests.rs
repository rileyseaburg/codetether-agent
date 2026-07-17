#[tokio::test]
async fn interrupted_ws_keeps_partial_private_and_disables_ws() {
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
        health.clone(),
        pool.clone(),
    )
    .collect::<Vec<_>>()
    .await;
    server.await.unwrap();
    assert!(chunks.is_empty(), "partial chunks must remain private");
    assert!(health.requires_http());
    assert!(pool.take().await.is_none());
}
