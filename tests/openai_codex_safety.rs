use codetether_agent::provider::openai_codex::{OAuthCredentials, OpenAiCodexProvider};
use codetether_agent::provider::{CompletionRequest, ContentPart, Message, Provider, Role};
use std::sync::OnceLock;
use tokio::sync::Mutex;

const OPT_IN_ENV: &str = "CODETETHER_OPENAI_CODEX_ALLOW_CHATGPT_BACKEND";

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

struct EnvGuard {
    old_value: Option<String>,
}

impl EnvGuard {
    fn without_opt_in() -> Self {
        let old_value = std::env::var(OPT_IN_ENV).ok();
        unsafe { std::env::remove_var(OPT_IN_ENV) };
        Self { old_value }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.old_value {
            Some(value) => unsafe { std::env::set_var(OPT_IN_ENV, value) },
            None => unsafe { std::env::remove_var(OPT_IN_ENV) },
        }
    }
}

fn request() -> CompletionRequest {
    CompletionRequest {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "hello".to_string(),
            }],
        }],
        tools: vec![],
        model: "gpt-5".to_string(),
        temperature: None,
        top_p: None,
        max_tokens: None,
        stop: vec![],
    }
}

#[tokio::test]
async fn chatgpt_backend_requires_explicit_opt_in() {
    let _lock = env_lock().lock().await;
    let _guard = EnvGuard::without_opt_in();
    let provider = OpenAiCodexProvider::from_credentials(OAuthCredentials {
        id_token: None,
        chatgpt_account_id: Some("org_test".to_string()),
        access_token: "oauth-access-token".to_string(),
        refresh_token: "oauth-refresh-token".to_string(),
        expires_at: u64::MAX,
    });

    let err = provider
        .complete(request())
        .await
        .expect_err("ChatGPT backend should require explicit opt-in");

    assert!(err.to_string().contains("OPENAI_API_KEY"));
    assert!(err.to_string().contains(OPT_IN_ENV));
}
