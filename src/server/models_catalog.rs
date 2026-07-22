//! OpenAI/OpenRouter-compatible model discovery handlers.

mod collect;
mod convert;
mod error;
mod ids;
mod parameters;
mod pricing;
mod query;
mod types;
mod types_architecture;
mod types_pricing;
mod types_top_provider;
mod types_vscode;
mod vscode;

use axum::{Json, Router, extract::Query, http::StatusCode, routing::get};

pub(crate) type ModelsResult<T> = Result<T, (StatusCode, Json<serde_json::Value>)>;

pub(crate) fn router() -> Router<super::AppState> {
    Router::new()
        .route("/models", get(list_models))
        .route("/v1/models", get(list_models))
        .route("/v1/models/vscode", get(list_vscode_models))
}

pub(crate) async fn list_models(
    Query(query): Query<query::ModelsQuery>,
) -> ModelsResult<Json<serde_json::Value>> {
    let data = model_data().await?;
    if vscode::requested(query.format.as_deref()) {
        return Ok(Json(serde_json::json!(vscode::response(&data))));
    }
    Ok(Json(serde_json::json!(types::ModelsResponse { data })))
}

pub(crate) async fn list_vscode_models() -> ModelsResult<Json<types_vscode::VscodeModelsResponse>> {
    Ok(Json(vscode::response(&model_data().await?)))
}

async fn model_data() -> ModelsResult<Vec<types::Model>> {
    let registry = crate::provider::ProviderRegistry::shared_from_vault()
        .await
        .map_err(error::load_error)?;
    Ok(collect::collect_models(&registry, chrono::Utc::now().timestamp()).await)
}
