//! One Responses WebSocket frame read with control-frame handling.

use anyhow::{Context, Result};
use futures::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_tungstenite::tungstenite::Message;

use super::super::OpenAiRealtimeConnection;

pub(super) enum Received {
    Event(Value),
    Activity,
    Closed,
    ClosedByServer,
}

pub(super) async fn next<S>(connection: &mut OpenAiRealtimeConnection<S>) -> Result<Received>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    loop {
        let Some(message) = connection.stream.next().await else {
            return Ok(Received::Closed);
        };
        match message.context("Responses WebSocket receive failed")? {
            Message::Text(text) => match serde_json::from_str(&text) {
                Ok(event) => return Ok(Received::Event(event)),
                Err(error) => {
                    tracing::debug!(%error, "Ignoring malformed WebSocket event");
                    return Ok(Received::Activity);
                }
            },
            Message::Binary(_) => anyhow::bail!("unexpected binary websocket event"),
            Message::Ping(payload) => connection.stream.send(Message::Pong(payload)).await?,
            Message::Pong(_) => {}
            Message::Frame(_) => return Ok(Received::Activity),
            Message::Close(_) => return Ok(Received::ClosedByServer),
        }
    }
}
