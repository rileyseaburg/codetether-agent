//! Backend endpoint selection for authenticated upgrade requests.
use crate::endpoints::{CHATGPT_CODEX_RESPONSES_WS_URL, OPENAI_RESPONSES_WS_URL};
use anyhow::Result;
use tokio_tungstenite::tungstenite::http::Request;

/// Build an authenticated upgrade request at the selected backend endpoint.
///
/// # Arguments
/// * `token` - Bearer token; blank values are rejected.
/// * `chatgpt_account_id` - Optional account header, ignored when blank.
/// # Returns
/// Upgrade request with the existing endpoint and header configuration.
/// # Errors
/// Returns invalid URL or header errors from the shared handshake builder.
pub fn build_responses_ws_request_with_account_id(
    token: &str,
    chatgpt_account_id: Option<&str>,
) -> Result<Request<()>> {
    super::build_responses_ws_request_with_base_url_and_account_id(
        OPENAI_RESPONSES_WS_URL,
        token,
        chatgpt_account_id,
    )
}

/// Build a ChatGPT upgrade request with bearer and optional account headers.
///
/// # Arguments
/// * `token` - Bearer token; blank values are rejected.
/// * `chatgpt_account_id` - Optional account header, ignored when blank.
/// # Returns
/// The authenticated upgrade request for the ChatGPT endpoint.
/// # Errors
/// Returns invalid URL or header errors from the shared handshake builder.
pub fn build_chatgpt_responses_ws_request(
    token: &str,
    chatgpt_account_id: Option<&str>,
) -> Result<Request<()>> {
    super::build_responses_ws_request_with_base_url_and_account_id(
        CHATGPT_CODEX_RESPONSES_WS_URL,
        token,
        chatgpt_account_id,
    )
}
