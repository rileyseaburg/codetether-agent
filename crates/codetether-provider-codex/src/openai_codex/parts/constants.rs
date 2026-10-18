const CHATGPT_BACKEND_OPT_IN_ENV: &str = "CODETETHER_OPENAI_CODEX_ALLOW_CHATGPT_BACKEND";
#[cfg(test)]
use codetether_codex_protocol::request::DEFAULT_RESPONSES_INSTRUCTIONS;
#[cfg(test)]
use codetether_codex_transport::X_CODEX_TURN_STATE_HEADER;

#[cfg(test)]
use codetether_codex_transport::endpoints::{
    CHATGPT_CODEX_API_URL, CHATGPT_CODEX_CLIENT_VERSION, CHATGPT_CODEX_RESPONSES_WS_URL,
    OPENAI_API_URL,
};
use codetether_codex_transport::endpoints::OPENAI_RESPONSES_WS_URL;
