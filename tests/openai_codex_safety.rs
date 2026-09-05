use codetether_agent::provider::Provider;
use codetether_agent::provider::openai_codex::{OAuthCredentials, OpenAiCodexProvider};

#[path = "codex_safety/environment.rs"]
mod environment;
#[path = "codex_safety/request.rs"]
mod fixture;
use environment::{EnvGuard, OPT_IN_ENV, env_lock};
use fixture::request;

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

    let result = provider.complete(request()).await;
    let err = result.expect_err("ChatGPT backend should require explicit opt-in");
    let stream_err = provider
        .complete_stream(request())
        .await
        .err()
        .expect("Streaming must also require explicit opt-in");
    for error in [err, stream_err] {
        let message = error.to_string();
        assert!(message.contains("backend is disabled"), "{error:#}");
        assert!(message.contains("OPENAI_API_KEY"), "{error:#}");
        assert!(message.contains(OPT_IN_ENV), "{error:#}");
    }
}
