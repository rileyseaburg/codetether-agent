use crate::approval::ApprovalStore;
use axum::{Json, extract::Path};
use serde_json::Value;

use super::{error, kind::DecisionKind, response, types::DecisionBody};

pub(super) async fn handle(
    Path(id): Path<String>,
    Json(body): Json<DecisionBody>,
) -> error::RouteResult<Value> {
    let kind = DecisionKind::parse(&body.decision).map_err(error::bad)?;
    let actor = body.actor.unwrap_or_else(|| "server".to_string());
    let reason = body
        .reason
        .unwrap_or_else(|| format!("{} from server", body.decision));
    let store = ApprovalStore::open_default().map_err(error::map)?;
    let receipt = if kind.approves() {
        let receipt = store.approve(&id, &actor, &reason).map_err(error::map)?;
        kind.grant_session(&receipt);
        Some(receipt)
    } else {
        store.deny(&id, &actor, &reason).map_err(error::map)?;
        None
    };
    let delivered = crate::approval::live::decide(&id, kind.live());
    let decision = store.decision(&id).map_err(error::map)?;
    Ok(Json(response::decided(
        &id,
        kind.status(),
        decision,
        receipt,
        delivered,
    )))
}
