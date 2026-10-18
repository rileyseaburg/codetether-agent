//! OpenAI and VS Code model discovery HTTP endpoints.
//!
//! Implement [`Host`] to load the application's provider registry and merge
//! [`router`] into the server. Discovery collection and projections remain in
//! [`crate::discovery`]; these modules own HTTP extraction and error responses.
mod data;
mod error;
mod handlers;
mod host;
mod routes;
use axum::{Json, http::StatusCode};
pub use host::Host;
pub use routes::router;
type ModelsResult<T> = Result<T, (StatusCode, Json<serde_json::Value>)>;
