use super::auth::ImagesAuth;
use anyhow::{Result, bail};

pub(super) async fn resolve() -> Result<ImagesAuth> {
    if let Some(key) = crate::secrets::get_api_key("openai-codex").await {
        return Ok(ImagesAuth::openai(key));
    }
    let disabled = std::env::var("CODETETHER_DISABLE_ENV_FALLBACK")
        .is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));
    if !disabled
        && let Ok(key) = std::env::var("OPENAI_API_KEY")
        && !key.trim().is_empty()
    {
        return Ok(ImagesAuth::openai(key));
    }
    if !disabled
        && let Some(auth) =
            crate::provider::openai_codex::OpenAiCodexProvider::local_chatgpt_backend_auth()
    {
        return Ok(ImagesAuth::chatgpt(auth));
    }
    bail!("image credentials unavailable; configure OpenAI or opted-in Codex authentication")
}
