//! HTTP receipts acknowledge local acceptance even without a connected worker.

use super::super::{contract::DispatchTaskRequest, service::enqueue};
use crate::{bus::AgentBus, server::KnativeTaskQueue};
use axum::{Json, body::to_bytes, http::StatusCode, response::IntoResponse};

#[tokio::test]
async fn receipt_does_not_claim_execution_without_subscribers() {
    let queue = KnativeTaskQueue::new();
    let bus = AgentBus::with_capacity(8).into_arc();
    let request: DispatchTaskRequest = serde_json::from_value(serde_json::json!({
        "title": "Queued locally", "description": "No executor acknowledgement"
    }))
    .unwrap();
    let response = Json(enqueue(&queue, &bus, request).await).into_response();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = to_bytes(response.into_body(), 8192).await.unwrap();
    let receipt: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(receipt["status"], "pending");
    assert_eq!(receipt["dispatch_mode"], "local_queue");
    assert_eq!(receipt["durable"], false);
    assert_eq!(receipt["dispatched_via_knative"], false);
    let task_id = receipt["task_id"].as_str().unwrap();
    assert!(uuid::Uuid::parse_str(task_id).is_ok());
    let queued = queue.get(task_id).await.unwrap();
    assert_eq!(queued.title, "Queued locally");
    assert_eq!(queued.status, "pending");
    assert_eq!(queue.list().await.len(), 1);
}

#[test]
fn dispatch_request_requires_title_and_description() {
    assert!(serde_json::from_value::<DispatchTaskRequest>(serde_json::json!({})).is_err());
}
