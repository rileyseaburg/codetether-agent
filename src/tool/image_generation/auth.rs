use crate::provider::openai_codex::ChatGptBackendAuth;
use reqwest::RequestBuilder;

pub(super) enum ImagesAuth {
    OpenAi { bearer: String, base_url: String },
    ChatGpt { bearer: String, account_id: String },
}

impl ImagesAuth {
    pub(super) fn openai(bearer: String) -> Self {
        let base_url =
            std::env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".into());
        Self::OpenAi {
            bearer,
            base_url: base_url.trim_end_matches('/').into(),
        }
    }

    pub(super) fn chatgpt(auth: ChatGptBackendAuth) -> Self {
        Self::ChatGpt {
            bearer: auth.access_token,
            account_id: auth.account_id,
        }
    }

    pub(super) fn endpoint(&self, path: &str) -> String {
        let base = match self {
            Self::OpenAi { base_url, .. } => base_url,
            Self::ChatGpt { .. } => "https://chatgpt.com/backend-api/codex",
        };
        format!("{base}/{path}")
    }

    pub(super) fn authorize(&self, request: RequestBuilder) -> RequestBuilder {
        match self {
            Self::OpenAi { bearer, .. } => request.bearer_auth(bearer),
            Self::ChatGpt { bearer, account_id } => request
                .bearer_auth(bearer)
                .header("chatgpt-account-id", account_id),
        }
    }
}
