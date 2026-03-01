use crate::provider::{CompletionRequest, ContentPart, Message, ProviderRegistry, Role};
use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::OnceCell;

#[derive(Debug, Deserialize)]
struct MorphResponse {
    choices: Vec<MorphChoice>,
}

#[derive(Debug, Deserialize)]
struct MorphChoice {
    message: MorphMessage,
}

#[derive(Debug, Deserialize)]
struct MorphMessage {
    #[serde(default)]
    content: Option<String>,
}

static MORPH_PROVIDER_REGISTRY: OnceCell<Arc<ProviderRegistry>> = OnceCell::const_new();

fn backend_enabled() -> bool {
    match std::env::var("CODETETHER_MORPH_TOOL_BACKEND") {
        Ok(v) => {
            let normalized = v.trim().to_ascii_lowercase();
            !matches!(normalized.as_str(), "0" | "false" | "no" | "off")
        }
        Err(_) => true,
    }
}

pub fn should_use_morph_backend() -> bool {
    backend_enabled()
}

fn morph_model_ref() -> String {
    let configured = std::env::var("CODETETHER_MORPH_TOOL_MODEL")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| "openrouter/morph/morph-v3-large".to_string());
    let trimmed = configured.trim();
    if trimmed.starts_with("openrouter/") {
        return trimmed.to_string();
    }
    if trimmed.starts_with("morph/") {
        return format!("openrouter/{trimmed}");
    }
    trimmed.to_string()
}

fn direct_openrouter_base_url() -> Option<String> {
    std::env::var("CODETETHER_OPENROUTER_BASE_URL")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.trim().trim_end_matches('/').to_string())
}

async fn get_registry() -> Result<Arc<ProviderRegistry>> {
    let registry = MORPH_PROVIDER_REGISTRY
        .get_or_try_init(|| async {
            let loaded = ProviderRegistry::from_vault().await?;
            Ok::<_, anyhow::Error>(Arc::new(loaded))
        })
        .await?;
    Ok(registry.clone())
}

async fn apply_with_provider(prompt: &str) -> Result<String> {
    let model_ref = morph_model_ref();
    let registry = get_registry()
        .await
        .context("Failed to initialize provider registry for Morph backend")?;
    let (provider, model) = registry
        .resolve_model(&model_ref)
        .with_context(|| format!("Failed to resolve Morph model '{model_ref}'"))?;

    tracing::info!(
        provider = %provider.name(),
        model = %model,
        "Executing Morph-backed edit via provider registry"
    );

    let request = CompletionRequest {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: prompt.to_string(),
            }],
        }],
        tools: vec![],
        model,
        temperature: None,
        top_p: None,
        max_tokens: None,
        stop: vec![],
    };

    let response = provider
        .complete(request)
        .await
        .context("Morph provider request failed")?;

    let content = response
        .message
        .content
        .into_iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    let content = content.trim().to_string();
    if content.is_empty() {
        return Err(anyhow!("Morph backend returned empty content"));
    }
    Ok(content)
}

async fn apply_with_direct_openrouter(prompt: &str, base_url: &str) -> Result<String> {
    let api_key = std::env::var("OPENROUTER_API_KEY").context(
        "OPENROUTER_API_KEY is required when CODETETHER_OPENROUTER_BASE_URL is set for direct Morph backend calls",
    )?;
    let model_ref = morph_model_ref();
    let model = if let Some(stripped) = model_ref.strip_prefix("openrouter/") {
        stripped.to_string()
    } else {
        model_ref
    };
    let body = json!({
        "model": model,
        "messages": [{
            "role": "user",
            "content": prompt
        }]
    });

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{base_url}/chat/completions"))
        .header("Authorization", format!("Bearer {api_key}"))
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "https://codetether.run")
        .header("X-Title", "CodeTether Agent")
        .json(&body)
        .send()
        .await
        .context("Failed to send Morph request")?;

    let status = resp.status();
    let text = resp.text().await.context("Failed to read Morph response")?;
    if !status.is_success() {
        anyhow::bail!("Morph backend error: {} {}", status, text);
    }

    let parsed: MorphResponse =
        serde_json::from_str(&text).context("Failed to parse Morph response JSON")?;
    let content = parsed
        .choices
        .first()
        .and_then(|c| c.message.content.clone())
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| anyhow!("Morph backend returned empty content"))?;

    Ok(content)
}

pub async fn apply_edit_with_morph(code: &str, instruction: &str, update: &str) -> Result<String> {
    let prompt = format!(
        "<instruction>{instruction}</instruction>\n<code>{code}</code>\n<update>{update}</update>"
    );
    if let Some(base_url) = direct_openrouter_base_url() {
        tracing::info!(
            base_url = %base_url,
            "Using direct OpenRouter HTTP path for Morph backend"
        );
        return apply_with_direct_openrouter(&prompt, &base_url).await;
    }
    apply_with_provider(&prompt).await
}
