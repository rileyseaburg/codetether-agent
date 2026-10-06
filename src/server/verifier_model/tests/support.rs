//! Isolated selection and routing observations for parallel-safe HTTP tests.

use super::super::{ApiState, defaults::Defaults};
pub(super) use super::request::call;
use crate::server::auth::{AuthState, require_auth};
use crate::tool::goal::verify::{VerifierSelection, observation::ObservationStore};
use axum::{Extension, Router, middleware};
use std::sync::Arc;

pub(super) fn setup() -> (Router, Arc<VerifierSelection>, Arc<ObservationStore>) {
    let selection = Arc::new(VerifierSelection::default());
    let observations = Arc::new(ObservationStore::default());
    let state = ApiState {
        selection: selection.clone(),
        observations: observations.clone(),
        defaults: Some(Defaults {
            environment: Some("openai-codex/environment".into()),
            configured: Some("default/model".into()),
        }),
    };
    let app = super::super::routes(state)
        .layer(middleware::from_fn(require_auth))
        .layer(Extension(AuthState::with_token("test-verifier-token")));
    (app, selection, observations)
}
