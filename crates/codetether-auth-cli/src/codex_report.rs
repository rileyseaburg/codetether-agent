//! Codex authentication completion and backend guidance.
use codetether_provider_codex::OAuthCredentials;

pub(super) fn print(
    credentials: &OAuthCredentials,
    api_key: Option<String>,
    expected_token_exchange_fallback: bool,
    chatgpt_account_id: Option<String>,
) {
    let expires_display = chrono::DateTime::from_timestamp(credentials.expires_at as i64, 0)
        .map(|ts| ts.to_rfc3339())
        .unwrap_or_else(|| credentials.expires_at.to_string());

    println!("Saved openai-codex credentials to HashiCorp Vault.");
    if api_key.is_some() {
        println!("Stored exchanged OpenAI API key for Codex model requests.");
    } else {
        println!(
            "Could not exchange an OpenAI API key; Codex requests will use ChatGPT OAuth backend tokens."
        );
        if expected_token_exchange_fallback {
            println!(
                "Note: this fallback is expected when your id_token does not include organization context."
            );
        }
    }
    if let Some(account_id) = chatgpt_account_id {
        println!("Using ChatGPT workspace/account ID: {}", account_id);
    }
    println!("Access token expires at {}", expires_display);
    println!("You can now select models like `openai-codex/gpt-5.5`.");
}
