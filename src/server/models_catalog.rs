//! OpenAI/OpenRouter-compatible model discovery handlers.

mod collect;
mod convert;
mod ids;
mod parameters;
mod pricing;
mod types;
mod types_architecture;
mod types_pricing;
mod types_top_provider;

use axum::{Json, http::StatusCode};

pub(crate) type ModelsResult<T> = Result<T, (StatusCode, Json<serde_json::Value>)>;

pub(crate) async fn list_models() -> ModelsResult<Json<types::ModelsResponse>> {
    let registry = crate::provider::ProviderRegistry::from_vault()
        .await
        .map_err(load_error)?;

    let created = chrono::Utc::now().timestamp();
    let data = collect::collect_models(&registry, created).await;
    Ok(Json(types::ModelsResponse { data }))
}

fn load_error(error: anyhow::Error) -> (StatusCode, Json<serde_json::Value>) {
    tracing::error!(error = %error, "Failed to load providers from Vault");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({
            "error": {
                "message": format!("failed to load providers: {error}"),
                "type": "server_error",
                "code": "internal_error"
            }
        })),
    )
}
