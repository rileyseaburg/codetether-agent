//! Model selection for the independent goal verifier.

/// Environment variable naming the verifier model, e.g. `openai/gpt-5`.
///
/// Setting it to a different model than the worker makes the second
/// opinion come from a different LLM.
pub const VERIFIER_MODEL_ENV: &str = "CODETETHER_GOAL_VERIFIER_MODEL";

/// Pick the verifier model from the available candidates.
///
/// Precedence: explicit override, then the worker's current model, then
/// the configured default. Blank values are skipped.
///
/// # Arguments
///
/// * `override_model` — Value of [`VERIFIER_MODEL_ENV`], if set.
/// * `current` — The model the worker agent is running on.
/// * `default` — The configured default model.
///
/// # Returns
///
/// The chosen model, or `None` when every candidate is missing or blank.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::goal::verify::select_verifier_model;
///
/// assert_eq!(select_verifier_model(Some("b/m2"), Some("a/m1"), None), Some("b/m2".into()));
/// assert_eq!(select_verifier_model(Some("  "), Some("a/m1"), None), Some("a/m1".into()));
/// assert_eq!(select_verifier_model(None, None, Some("c/m3")), Some("c/m3".into()));
/// assert_eq!(select_verifier_model(None, None, None), None);
/// ```
pub fn select_verifier_model(
    override_model: Option<&str>,
    current: Option<&str>,
    default: Option<&str>,
) -> Option<String> {
    [override_model, current, default]
        .into_iter()
        .flatten()
        .map(str::trim)
        .find(|model| !model.is_empty())
        .map(str::to_string)
}

/// Resolve the verifier model from the TUI selection, environment, worker, and config.
///
/// Precedence: TUI selection ([`set_verifier_model`](super::set_verifier_model)),
/// then [`VERIFIER_MODEL_ENV`], then the worker model, then the configured default.
///
/// # Errors
///
/// Returns an error if configuration cannot be loaded or no model is found.
///
/// # Examples
///
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use codetether_agent::tool::goal::verify::resolve_verifier_model;
///
/// let model = resolve_verifier_model(Some("openai/gpt-5")).await.unwrap();
/// assert!(!model.is_empty());
/// # });
/// ```
pub async fn resolve_verifier_model(current: Option<&str>) -> anyhow::Result<String> {
    let override_model =
        super::selected_verifier_model().or_else(|| std::env::var(VERIFIER_MODEL_ENV).ok());
    let config = crate::config::Config::load().await?;
    select_verifier_model(
        override_model.as_deref(),
        current,
        config.default_model.as_deref(),
    )
    .ok_or_else(|| {
        anyhow::anyhow!("no model available for the goal verifier; set {VERIFIER_MODEL_ENV}")
    })
}
