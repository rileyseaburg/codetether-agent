const OPENAI_API_URL: &str = "https://api.openai.com/v1";
// `chatgpt.com/backend-api/codex` is not the public OpenAI API. Keep it behind
// an explicit opt-in and prefer `OPENAI_API_KEY`/`api.openai.com` when present.
const CHATGPT_CODEX_API_URL: &str = "https://chatgpt.com/backend-api/codex";
const OPENAI_RESPONSES_WS_URL: &str = "wss://api.openai.com/v1/responses";
const CHATGPT_CODEX_RESPONSES_WS_URL: &str = "wss://chatgpt.com/backend-api/codex/responses";
// Latest openai/codex release (rust-v0.159.1, 2026-09-29). The backend gates
// both `/models` listings and `/responses` access on this version: with
// 0.153.3, gpt-6.1-sol was hidden from `/models` and rejected by
// `/responses`; with 0.159.1 it is listed and answers (verified live).
const CHATGPT_CODEX_CLIENT_VERSION: &str = "0.159.1";
const AUTH_ISSUER: &str = "https://auth.openai.com";
const AUTHORIZE_URL: &str = "https://auth.openai.com/oauth/authorize";
const TOKEN_URL: &str = "https://auth.openai.com/oauth/token";
// Public clients cannot treat this OAuth client ID as a secret. Backend use is
// still gated by `CODETETHER_OPENAI_CODEX_ALLOW_CHATGPT_BACKEND`.
const CLIENT_ID: &str = "app_EMoamEEZ73f0CkXaXp7hrann";
const REDIRECT_URI: &str = "http://localhost:1455/auth/callback";
const SCOPE: &str = "openid profile email offline_access";
const CHATGPT_BACKEND_OPT_IN_ENV: &str = "CODETETHER_OPENAI_CODEX_ALLOW_CHATGPT_BACKEND";
const X_CODEX_TURN_STATE_HEADER: &str = "x-codex-turn-state";
const DEFAULT_RESPONSES_INSTRUCTIONS: &str = "You are CodeTether Agent running on OpenAI Codex. \
Resolve software tasks directly: inspect the workspace, make focused changes, validate with \
available tools, and report concise results. When model availability or external APIs are involved, \
verify live behavior before treating it as supported.";
