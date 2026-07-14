//! OpenAI Codex provider backed by OpenAI API keys or ChatGPT OAuth.
//!
//! [`OpenAiCodexProvider`] implements [`Provider`] and supports OAuth token
//! exchange, Responses API streaming, and WebSocket-to-HTTP recovery. The
//! implementation is split by responsibility under `openai_codex/`.
//!
//! # Example
//!
//! ```
//! use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
//!
//! let provider = OpenAiCodexProvider::from_api_key("test-key".to_string());
//! assert_eq!(format!("{provider:?}").contains("has_api_key: true"), true);
//! ```

#[path = "openai_codex/reasoning_catalog.rs"]
pub mod reasoning_catalog;
#[path = "openai_codex/runtime_config.rs"]
pub mod runtime_config;
#[path = "openai_codex/service_tier_catalog.rs"]
pub(crate) mod service_tier_catalog;
#[path = "openai_codex/stream_recovery.rs"]
mod stream_recovery;
#[path = "openai_codex/thinking_level.rs"]
mod thinking_level;
#[path = "openai_codex/token_expiry.rs"]
mod token_expiry;
#[path = "openai_codex/transport_catalog.rs"]
mod transport_catalog;
#[path = "openai_codex/transport_health.rs"]
mod transport_health;
#[path = "openai_codex/ws_stream.rs"]
mod ws_stream;

include!("openai_codex/parts_manifest_a.rs");
include!("openai_codex/parts_manifest_b.rs");
include!("openai_codex/parts_manifest_c.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream;
    use tokio::io::duplex;
    use tokio_tungstenite::{
        accept_hdr_async, client_async,
        tungstenite::{
            Message as WsMessage,
            handshake::server::{Request as ServerRequest, Response as ServerResponse},
        },
    };

    include!("openai_codex_basic_tests.rs");
    include!("openai_codex_reasoning_tests.rs");
    include!("openai_codex_stream_recovery_tests.rs");
    include!("openai_codex_ws_completion_tests.rs");
    include!("openai_codex_ws_interruption_tests.rs");
    include!("openai_codex/tests_manifest.rs");
}
