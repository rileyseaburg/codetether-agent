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

include!("openai_codex/module_manifest.rs");

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
    include!("openai_codex_ws_reuse_tests.rs");
    include!("openai_codex/tests_manifest.rs");
}
