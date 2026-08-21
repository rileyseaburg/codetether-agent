//! Concrete split WebSocket types shared by realtime modules.

use axum::extract::ws::{Message, WebSocket};
use futures::stream::{SplitSink, SplitStream};

/// Sending half of one Axum WebSocket.
pub(super) type SocketSink = SplitSink<WebSocket, Message>;

/// Receiving half of one Axum WebSocket.
pub(super) type SocketStream = SplitStream<WebSocket>;
