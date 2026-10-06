//! HTTP parsing and responses; persistence and identity remain in the harness.

use super::{
    error, input,
    state::ApiState,
    types::{HttpError, Query as ModelQuery, SetModel, Settings},
};
use axum::{Extension, Json, extract::Query};

pub(super) async fn read(
    Extension(state): Extension<ApiState>,
    Query(query): Query<ModelQuery>,
) -> Result<Json<Settings>, HttpError> {
    let worker = query
        .worker_model
        .map(|model| input::model(&model, false))
        .transpose()?;
    let defaults = state.defaults().await.map_err(error::configuration)?;
    Ok(Json(state.snapshot(defaults, worker)))
}

pub(super) async fn replace(
    Extension(state): Extension<ApiState>,
    Json(request): Json<SetModel>,
) -> Result<Json<Settings>, HttpError> {
    let model = input::model(&request.model, true)?;
    let defaults = state.defaults().await.map_err(error::configuration)?;
    state.selection.set(Some(&model));
    Ok(Json(state.snapshot(defaults, None)))
}

pub(super) async fn clear(
    Extension(state): Extension<ApiState>,
) -> Result<Json<Settings>, HttpError> {
    let defaults = state.defaults().await.map_err(error::configuration)?;
    state.selection.set(None);
    Ok(Json(state.snapshot(defaults, None)))
}
