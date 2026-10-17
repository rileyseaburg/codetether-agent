//! Upstream transport endpoints and the pinned Codex client version.
/// Public OpenAI HTTP API base.
pub const OPENAI_API_URL: &str = "https://api.openai.com/v1";

// `chatgpt.com/backend-api/codex` is not the public OpenAI API. Keep it behind
// an explicit opt-in and prefer `OPENAI_API_KEY`/`api.openai.com` when present.
/// Opt-in ChatGPT Codex HTTP API base.
pub const CHATGPT_CODEX_API_URL: &str = "https://chatgpt.com/backend-api/codex";

/// Public OpenAI Responses WebSocket endpoint.
pub const OPENAI_RESPONSES_WS_URL: &str = "wss://api.openai.com/v1/responses";

/// Opt-in ChatGPT Codex Responses WebSocket endpoint.
pub const CHATGPT_CODEX_RESPONSES_WS_URL: &str = "wss://chatgpt.com/backend-api/codex/responses";

// Latest openai/codex release (rust-v0.159.1, 2026-09-29). The backend gates
// both `/models` listings and `/responses` access on this version: with
// 0.153.3, gpt-6.1-sol was hidden from `/models` and rejected by
// `/responses`; with 0.159.1 it is listed and answers (verified live).
/// Pinned version header expected by the existing ChatGPT backend integration.
pub const CHATGPT_CODEX_CLIENT_VERSION: &str = "0.159.1";
