//! Model-selection policy for A2A sessions.
//!
//! [configured_model] prefers a nonempty environment override and otherwise
//! loads configuration on demand. The application applies the returned model.
mod configured;
mod environment;
use configured::async_load_config_model;
use environment::model_from_env;

/// Select the model for an A2A conversation.
///
/// # Arguments
/// No arguments; reads the environment and current configuration.
/// # Returns
/// A configured model or `None`; configuration failures are logged.
/// # Examples
/// ```rust,no_run
/// # async fn demo() {
/// let _model = codetether_config::a2a_runtime::configured_model().await;
/// # }
/// ```

pub async fn configured_model() -> Option<String> {
    match model_from_env() {
        Some(model) => Some(model),
        None => async_load_config_model().await,
    }
}

mod worker_access;
pub use worker_access::auto_approve_access_mode;
