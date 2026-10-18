//! Error helpers for model catalog endpoints.

use axum::{Json, http::StatusCode};

pub(crate) fn load_error(error: anyhow::Error) -> (StatusCode, Json<serde_json::Value>) {
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
