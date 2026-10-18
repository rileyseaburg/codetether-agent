//! Ordinary-run model selection and session assignment.
use crate::SessionPolicy;
use codetether_cli_options::RunArgs;
/// Apply the CLI, environment or configuration model in that order.
/// # Arguments
/// * `session` - Session receiving the selected model.
/// * `args` - Explicit model option.
/// * `config_model` - Optional configuration fallback.
/// # Returns
/// Unit after logging and applying a selected model, if any.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_run_policy::SessionPolicy>(session: &mut S, args: &codetether_cli_options::RunArgs) {
/// codetether_run_policy::apply_model(session, args, None);
/// # }
/// ```
pub fn apply_model<S: SessionPolicy>(
    session: &mut S,
    args: &RunArgs,
    config_model: Option<String>,
) {
    let model = args
        .model
        .clone()
        .or_else(|| std::env::var("CODETETHER_DEFAULT_MODEL").ok())
        .or(config_model);

    if let Some(model) = model {
        tracing::info!("Using model: {}", model);
        session.set_model(model);
    }
}
