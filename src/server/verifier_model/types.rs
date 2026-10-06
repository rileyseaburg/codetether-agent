//! Typed verifier settings and harness-reported execution identity.

use crate::tool::goal::verify::observation::Observation;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SetModel {
    pub model: String,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Query {
    pub worker_model: Option<String>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(super) enum Source {
    Runtime,
    Environment,
    Default,
    Worker,
    Unconfigured,
}

#[derive(Serialize)]
pub(super) struct Settings {
    pub identity_source: &'static str,
    pub scope: &'static str,
    pub persisted: bool,
    pub runtime_model: Option<String>,
    pub environment_model: Option<String>,
    pub default_model: Option<String>,
    pub worker_model_context: Option<String>,
    pub selected_model: Option<String>,
    pub source: Source,
    pub latest_verification: Option<Observation>,
}

#[derive(Serialize)]
pub(super) struct ApiError {
    pub error: &'static str,
}
pub(super) type HttpError = (axum::http::StatusCode, axum::Json<ApiError>);
