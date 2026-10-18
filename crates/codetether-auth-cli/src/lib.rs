//! Authentication command routing and provider-specific OAuth orchestration.
//!
//! [execute] dispatches [AuthArgs], preserving caller-supplied version metadata.
//! Account, Bedrock, cookie, device and callback operations use their dedicated
//! crates; Codex/Copilot token storage and user instructions live here.
mod args;
mod client_id;
mod codex;
mod codex_device;
mod codex_fallback;
mod codex_request;
mod codex_store;
mod copilot;
mod dispatch;
mod prompt;
mod provider_args;
pub use args::{AuthArgs, AuthCommand};
use client_id::resolve_client_id;
use codetether_cookie_auth_cli::CookieAuthArgs;
use codetether_device_auth::{
    poll_for_access_token, poll_for_codex_authorization_code, request_device_code,
};
use codetether_oauth_callback::{capture_oauth_callback_auto, extract_oauth_code_and_state};
use codetether_user_auth_cli::{LoginAuthArgs, RegisterAuthArgs};
use codex::authenticate_codex;
use codex_device::authenticate_codex_device_code;
use codex_fallback::is_expected_codex_id_token_exchange_fallback;
use codex_request::request_codex_device_code;
use codex_store::store_codex_credentials;
use copilot::authenticate_copilot;
pub use dispatch::execute;
use prompt::{prompt_line, prompt_optional_line};
pub use provider_args::{CodexAuthArgs, CopilotAuthArgs};

const DEFAULT_GITHUB_DOMAIN: &str = "github.com";

const CODEX_CALLBACK_TIMEOUT_SECS: u64 = 300;

const CODEX_CALLBACK_TIMEOUT_SSH_SECS: u64 = 15;

mod codex_account;
mod codex_browser;
mod codex_callback;
mod codex_exchange;
mod codex_instructions;
mod codex_metadata;
mod codex_report;
mod copilot_instructions;
mod copilot_save;
mod copilot_target;
