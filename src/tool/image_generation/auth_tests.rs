use super::auth::ImagesAuth;
use crate::provider::openai_codex::ChatGptBackendAuth;

#[test]
fn chatgpt_auth_routes_to_codex_backend() {
    let auth = ImagesAuth::chatgpt(ChatGptBackendAuth {
        access_token: "token".into(),
        account_id: "account".into(),
    });
    assert_eq!(
        auth.endpoint("images/generations"),
        "https://chatgpt.com/backend-api/codex/images/generations"
    );
}

#[test]
fn openai_auth_routes_to_public_api() {
    let auth = ImagesAuth::OpenAi {
        bearer: "key".into(),
        base_url: "https://api.openai.com/v1".into(),
    };
    assert_eq!(
        auth.endpoint("images/generations"),
        "https://api.openai.com/v1/images/generations"
    );
}
