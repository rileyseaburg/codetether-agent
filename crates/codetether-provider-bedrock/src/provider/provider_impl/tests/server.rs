//! Local HTTP fixture exercising the production byte-to-stream decoder.
use codetether_provider_core::StreamChunk;
use futures::stream::BoxStream;
use std::{net::TcpListener, sync::mpsc, thread, time::Duration};

pub(super) struct Fixture {
    pub stream: BoxStream<'static, StreamChunk>,
    pub release: mpsc::Sender<()>,
    pub task: thread::JoinHandle<()>,
}

pub(super) async fn open(first: Vec<u8>, tail: Vec<u8>) -> Fixture {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/converse-stream", listener.local_addr().unwrap());
    let (release, gate) = mpsc::channel();
    let task = thread::spawn(move || super::server_io::serve(listener, first, tail, gate));
    let response = reqwest::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap()
        .get(url)
        .send()
        .await
        .unwrap();
    Fixture {
        stream: codetether_bedrock_protocol::stream::event_chunk_stream(response),
        release,
        task,
    }
}

pub(super) async fn next(stream: &mut BoxStream<'static, StreamChunk>) -> StreamChunk {
    use futures::StreamExt;
    tokio::time::timeout(Duration::from_secs(2), stream.next())
        .await
        .unwrap()
        .unwrap()
}
