// Simulate two sequential responses over the same test connection.
fn start_reuse_server(server_io: tokio::io::DuplexStream) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
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
    })
}
