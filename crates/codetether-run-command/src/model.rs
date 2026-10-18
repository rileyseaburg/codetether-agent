//! Model precedence for CLI relay requests.

// Easy-go defaults align with the TUI /go rotation model.
const GO_DEFAULT_MODEL: &str = "minimax/MiniMax-M3";

/// Choose a model using CLI, easy-go, environment and configuration precedence.
/// # Arguments
/// * `cli_model` - Explicit model override.
/// * `env_model` - Environment model, if present.
/// * `config_model` - Configured model, if present.
/// * `easy_go_requested` - Whether go/team default policy applies.
/// # Returns
/// The selected nonblank model or the existing fallback.
/// # Examples
/// ```
/// assert_eq!(codetether_run_command::resolve_autochat_model(None, None, None, true), "minimax/MiniMax-M3");
/// ```
pub fn resolve_autochat_model(
    cli_model: Option<&str>,
    env_model: Option<&str>,
    config_model: Option<&str>,
    easy_go_requested: bool,
) -> String {
    if let Some(model) = cli_model.filter(|value| !value.trim().is_empty()) {
        return model.to_string();
    }
    if easy_go_requested {
        return GO_DEFAULT_MODEL.to_string();
    }
    if let Some(model) = env_model.filter(|value| !value.trim().is_empty()) {
        return model.to_string();
    }
    if let Some(model) = config_model.filter(|value| !value.trim().is_empty()) {
        return model.to_string();
    }
    "zai/glm-5".to_string()
}
