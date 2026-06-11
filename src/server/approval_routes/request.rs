use crate::approval::ApprovalStore;
use axum::Json;
use serde_json::Value;

use super::{error, response, types::RequestBody};

pub(super) async fn handle(Json(body): Json<RequestBody>) -> error::RouteResult<Value> {
    let reason = body
        .reason
        .unwrap_or_else(|| "server approval request".to_string());
    let store = ApprovalStore::open_default().map_err(error::map)?;
    let request = store
        .create_request(&body.tool, &body.action, &body.resource, &reason)
        .map_err(error::map)?;
    Ok(Json(response::pending(&request, body.tool_call_id)))
}
