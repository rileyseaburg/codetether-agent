//! Resolve the account identifier attached to Codex credentials.
use codetether_provider_codex::{OAuthCredentials, OpenAiCodexProvider};

pub(super) fn id(credentials: &OAuthCredentials) -> Option<String> {
    let chatgpt_account_id = credentials
        .chatgpt_account_id
        .clone()
        .or_else(|| {
            credentials
                .id_token
                .as_deref()
                .and_then(OpenAiCodexProvider::extract_chatgpt_account_id)
        })
        .or_else(|| OpenAiCodexProvider::extract_chatgpt_account_id(&credentials.access_token));

    chatgpt_account_id
}
