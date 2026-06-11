use crate::approval::ApprovalStore;
use serde_json::Value;

use super::{JsonRpcError, kind::DecisionKind, params, response};

pub(super) fn handle(params: Option<Value>) -> Result<Value, JsonRpcError> {
    let params: params::DecisionParams = params::parse(params)?;
    let kind = DecisionKind::parse(&params.decision).map_err(JsonRpcError::invalid_params)?;
    let actor = params.actor.unwrap_or_else(|| "mcp".to_string());
    let reason = params
        .reason
        .unwrap_or_else(|| format!("{} from mcp", params.decision));
    let store = ApprovalStore::open_default()
        .map_err(|error| JsonRpcError::internal_error(error.to_string()))?;
    let receipt = if kind.approves() {
        let receipt = store
            .approve(&params.approval_id, &actor, &reason)
            .map_err(|error| JsonRpcError::invalid_params(error.to_string()))?;
        kind.grant_session(&receipt);
        Some(receipt)
    } else {
        store
            .deny(&params.approval_id, &actor, &reason)
            .map_err(|error| JsonRpcError::invalid_params(error.to_string()))?;
        None
    };
    let delivered = crate::approval::live::decide(&params.approval_id, kind.live());
    let decision = store
        .decision(&params.approval_id)
        .map_err(|error| JsonRpcError::internal_error(error.to_string()))?;
    Ok(response::decided(
        &params.approval_id,
        kind.status(),
        decision,
        receipt,
        delivered,
    ))
}
