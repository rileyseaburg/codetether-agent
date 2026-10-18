//! Model discovery response-format selection and HTTP extraction.
use super::{Host, ModelsResult, data::model_data};
use crate::discovery::{query, types, types_vscode, vscode};
use axum::{Json, extract::Query};

pub(super) async fn list_models<H: Host>(
    Query(query): Query<query::ModelsQuery>,
) -> ModelsResult<Json<serde_json::Value>> {
    let data = model_data::<H>().await?;
    if vscode::requested(query.format.as_deref()) {
        return Ok(Json(serde_json::json!(vscode::response(&data))));
    }
    Ok(Json(serde_json::json!(types::ModelsResponse { data })))
}

pub(super) async fn list_vscode_models<H: Host>()
-> ModelsResult<Json<types_vscode::VscodeModelsResponse>> {
    Ok(Json(vscode::response(&model_data::<H>().await?)))
}
