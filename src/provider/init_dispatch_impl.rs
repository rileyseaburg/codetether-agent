//! Per-provider dispatch helpers called from [`super::init_dispatch`].
//!
//! Each function handles provider-specific secret schemas (service accounts,
//! OAuth tokens, model paths, etc.) that don't fit the generic `api_key` pattern.

use super::init_dispatch;
use crate::provider::traits::Provider;
use crate::secrets::ProviderSecrets;
use std::sync::Arc;

pub(super) fn dispatch_bedrock(secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    use crate::provider::bedrock;
    let region = secrets
        .extra
        .get("region")
        .and_then(|v| v.as_str())
        .unwrap_or("us-east-1")
        .to_string();
    let key_id = secrets
        .extra
        .get("aws_access_key_id")
        .and_then(|v| v.as_str());
    let secret = secrets
        .extra
        .get("aws_secret_access_key")
        .and_then(|v| v.as_str());
    let result = if let (Some(kid), Some(sec)) = (key_id, secret) {
        let creds = bedrock::AwsCredentials {
            access_key_id: kid.into(),
            secret_access_key: sec.into(),
            session_token: secrets
                .extra
                .get("aws_session_token")
                .and_then(|v| v.as_str())
                .map(String::from),
        };
        bedrock::BedrockProvider::with_credentials(creds, region)
    } else if let Some(ref key) = secrets.api_key {
        bedrock::BedrockProvider::with_region(key.clone(), region)
    } else if let Some(creds) = bedrock::AwsCredentials::from_environment() {
        bedrock::BedrockProvider::with_credentials(creds, region)
    } else {
        tracing::warn!("No AWS credentials or API key found for Bedrock");
        return None;
    };
    match result {
        Ok(p) => Some(Arc::new(p)),
        Err(e) => {
            tracing::warn!("Failed to init bedrock: {e}");
            None
        }
    }
}

pub(super) fn dispatch_vertex_glm(secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    use crate::provider::vertex_glm;
    let sa = secrets
        .extra
        .get("service_account_json")
        .and_then(|v| v.as_str())?;
    let pid = secrets
        .extra
        .get("project_id")
        .and_then(|v| v.as_str())
        .or_else(|| secrets.extra.get("projectId").and_then(|v| v.as_str()))
        .map(String::from);
    match vertex_glm::VertexGlmProvider::new(sa, pid) {
        Ok(p) => Some(Arc::new(p)),
        Err(e) => {
            tracing::warn!("Failed to init vertex-glm: {e}");
            None
        }
    }
}

pub(super) fn dispatch_vertex_anthropic(secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    use crate::provider::vertex_anthropic;
    let sa = secrets
        .extra
        .get("service_account_json")
        .and_then(|v| v.as_str())?;
    let pid = secrets
        .extra
        .get("project_id")
        .and_then(|v| v.as_str())
        .or_else(|| secrets.extra.get("projectId").and_then(|v| v.as_str()))
        .map(String::from);
    match vertex_anthropic::VertexAnthropicProvider::new(sa, pid) {
        Ok(p) => Some(Arc::new(p)),
        Err(e) => {
            tracing::warn!("Failed to init vertex-anthropic: {e}");
            None
        }
    }
}

pub(super) fn dispatch_codex(secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    use crate::provider::openai_codex;
    if let Some(key) = secrets.api_key.as_deref().filter(|k| !k.is_empty()) {
        return Some(Arc::new(openai_codex::OpenAiCodexProvider::from_api_key(
            key.into(),
        )));
    }
    let access = secrets.extra.get("access_token").and_then(|v| v.as_str())?;
    let refresh = secrets
        .extra
        .get("refresh_token")
        .and_then(|v| v.as_str())?;
    let expires = secrets.extra.get("expires_at").and_then(|v| v.as_u64())?;
    let creds = openai_codex::OAuthCredentials {
        id_token: secrets
            .extra
            .get("id_token")
            .and_then(|v| v.as_str())
            .map(String::from),
        chatgpt_account_id: secrets
            .extra
            .get("chatgpt_account_id")
            .and_then(|v| v.as_str())
            .map(String::from),
        access_token: access.into(),
        refresh_token: refresh.into(),
        expires_at: expires,
    };
    Some(Arc::new(
        openai_codex::OpenAiCodexProvider::from_credentials(creds),
    ))
}

pub(super) fn dispatch_gemini_web(secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    use crate::provider::gemini_web;
    let cookies = secrets.extra.get("cookies").and_then(|v| v.as_str())?;
    match gemini_web::GeminiWebProvider::new(cookies.into()) {
        Ok(p) => Some(Arc::new(p)),
        Err(e) => {
            tracing::warn!("Failed to init gemini-web: {e}");
            None
        }
    }
}

pub(super) fn dispatch_local_cuda(secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    use crate::provider::local_cuda;
    let name = secrets
        .extra
        .get("model_name")
        .and_then(|v| v.as_str())
        .or_else(|| secrets.extra.get("model").and_then(|v| v.as_str()))
        .unwrap_or("qwen3.5-9b")
        .to_string();
    let path = secrets
        .extra
        .get("model_path")
        .and_then(|v| v.as_str())
        .map(String::from);
    let tok_path = secrets
        .extra
        .get("tokenizer_path")
        .and_then(|v| v.as_str())
        .map(String::from);
    let arch = secrets
        .extra
        .get("architecture")
        .and_then(|v| v.as_str())
        .map(String::from);
    let result = if let Some(p) = path {
        local_cuda::LocalCudaProvider::with_paths(name.clone(), p, tok_path, arch)
    } else {
        local_cuda::LocalCudaProvider::new(name.clone())
    };
    match result {
        Ok(p) => {
            tracing::info!(model = %name, "Registered Local CUDA from Vault");
            Some(Arc::new(p))
        }
        Err(e) => {
            tracing::warn!("Failed to init local-cuda: {e}");
            None
        }
    }
}

pub(super) fn dispatch_huggingface(secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    use crate::provider::openai;
    let url = secrets.base_url.as_ref()?;
    match openai::OpenAIProvider::with_base_url_optional_key(
        secrets.api_key.clone(),
        url.clone(),
        "huggingface",
    ) {
        Ok(p) => Some(Arc::new(p)),
        Err(e) => {
            tracing::warn!("Failed to init huggingface: {e}");
            None
        }
    }
}

pub(super) fn dispatch_copilot(
    key: &str,
    secrets: &ProviderSecrets,
    name: &str,
) -> Option<Arc<dyn Provider>> {
    use crate::provider::copilot;
    let result = if let Some(url) = &secrets.base_url {
        copilot::CopilotProvider::with_base_url(key.into(), url.clone(), name)
    } else {
        copilot::CopilotProvider::new(key.into())
    };
    match result {
        Ok(p) => Some(Arc::new(p)),
        Err(e) => {
            tracing::warn!("Failed to init {name}: {e}");
            None
        }
    }
}

pub(super) fn dispatch_copilot_enterprise(
    key: &str,
    secrets: &ProviderSecrets,
) -> Option<Arc<dyn Provider>> {
    use crate::provider::copilot;
    let ent_url = secrets
        .extra
        .get("enterpriseUrl")
        .and_then(|v| v.as_str())
        .or_else(|| secrets.extra.get("enterprise_url").and_then(|v| v.as_str()));
    let result = if let Some(url) = &secrets.base_url {
        copilot::CopilotProvider::with_base_url(
            key.into(),
            url.clone(),
            "github-copilot-enterprise",
        )
    } else if let Some(u) = ent_url {
        copilot::CopilotProvider::enterprise(key.into(), u.into())
    } else {
        copilot::CopilotProvider::with_base_url(
            key.into(),
            "https://api.githubcopilot.com".into(),
            "github-copilot-enterprise",
        )
    };
    match result {
        Ok(p) => Some(Arc::new(p)),
        Err(e) => {
            tracing::warn!("Failed to init github-copilot-enterprise: {e}");
            None
        }
    }
}

pub(super) fn dispatch_glm5(key: &str, secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    use crate::provider::glm5;
    let url = secrets.base_url.clone().unwrap_or_else(|| {
        std::env::var("GLM5_BASE_URL").unwrap_or_else(|_| "https://localhost:18000/v1".into())
    });
    let model = secrets
        .extra
        .get("model_name")
        .and_then(|v| v.as_str())
        .unwrap_or(glm5::DEFAULT_MODEL)
        .to_string();
    match glm5::Glm5Provider::with_model(key.into(), url, model) {
        Ok(p) => Some(Arc::new(p)),
        Err(e) => {
            tracing::warn!("Failed to init glm5: {e}");
            None
        }
    }
}

pub(super) fn dispatch_openai_compat(
    key: &str,
    secrets: &ProviderSecrets,
    pid: &str,
) -> Option<Arc<dyn Provider>> {
    use crate::provider::openai;
    if let Some(url) = &secrets.base_url {
        match openai::OpenAIProvider::with_base_url(key.into(), url.clone(), pid) {
            Ok(p) => return Some(Arc::new(p)),
            Err(e) => {
                tracing::warn!("Failed to init {pid}: {e}");
                return None;
            }
        }
    }
    if pid == "openai" {
        match openai::OpenAIProvider::new(key.into()) {
            Ok(p) => return Some(Arc::new(p)),
            Err(e) => {
                tracing::warn!("Failed to init openai: {e}");
                return None;
            }
        }
    }
    if pid == "novita" {
        let url = "https://api.novita.ai/openai/v1".to_string();
        match openai::OpenAIProvider::with_base_url(key.into(), url, pid) {
            Ok(p) => return Some(Arc::new(p)),
            Err(e) => {
                tracing::warn!("Failed to init {pid}: {e}");
                return None;
            }
        }
    }
    tracing::warn!("Provider {pid} has no built-in base_url; set base_url in Vault");
    None
}
