//! Codex transport streaming and bounded request-opening retries.
//!
//! [drive_responses_http_stream] consumes one HTTP response using the shared
//! protocol parser. [with_http_retry] retries only before a stream is returned.
//! Provider adapters supply credential-aware request factories.
use codetether_provider_core::StreamChunk;
use futures::stream::BoxStream;
/// Stream of provider chunks from a Codex transport.
pub type Chunks = BoxStream<'static, StreamChunk>;
mod attempt;
mod classify;
mod delay;
mod http_stream;
mod log;
mod retry;
pub use classify::{is_unauthorized, is_upgrade_required, tag_stream_open};
pub use http_stream::drive_responses_http_stream;
pub use retry::with_http_retry;

mod connection;
pub mod transport_health;
pub mod turn_state;
pub mod websocket;
mod ws_pool;
pub use connection::OpenAiRealtimeConnection;
pub use transport_health::TransportHealth;
pub use turn_state::TurnStateStore;
pub use ws_pool::WsPool;
/// Header carrying sticky routing state for a Codex turn.
pub const X_CODEX_TURN_STATE_HEADER: &str = "x-codex-turn-state";

use codetether_codex_protocol::ResponsesSseParser;

pub mod endpoints;
mod http_client;
mod http_response;
pub mod request_log;
pub use http_client::HttpTransport;
pub use http_response::{format_openai_api_error, validate_http_response};

mod model_discovery;
pub use model_discovery::discover_chatgpt_models;

mod websocket_client;
pub use websocket_client::WebSocketTransport;
mod collect;
pub use collect::collect_stream_completion;
