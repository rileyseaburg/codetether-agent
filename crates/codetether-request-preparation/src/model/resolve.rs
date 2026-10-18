use super::{ModelState, SelectionHost};
/// Resolve a session selector and construct its initial model state.
/// # Arguments
/// `session` supplies native model metadata; `host` supplies configured providers
/// and builds the selected model's tool and prompt settings.
/// # Returns
/// A provider client and complete initial model state.
/// # Errors
/// Returns an error for an empty registry, unavailable explicit provider or
/// missing selected provider.
/// # Examples
/// ```no_run
/// # fn configure<R, S>(session: &S, host: &impl codetether_request_preparation::model::SelectionHost<R, S>) -> anyhow::Result<()> {
/// let _model = codetether_request_preparation::model::resolve(session, host)?;
/// # Ok(()) }
/// ```
pub fn resolve<R, S>(
    session: &S,
    host: &impl SelectionHost<R, S>,
) -> anyhow::Result<ModelState<R>> {
    let providers = host.providers();
    if providers.is_empty() {
        anyhow::bail!(
            "No providers available. Configure provider credentials in HashiCorp Vault (for ChatGPT subscription Codex use `codetether auth codex`; for Copilot use `codetether auth copilot`)."
        );
    }
    let (requested, requested_model) =
        super::selector::parse(host.original_model(session), &providers);
    let name = codetether_provider_policy::resolve_provider_for_session_request(
        &providers,
        requested.as_deref(),
    )?
    .to_string();
    let provider = host
        .provider(&name)
        .ok_or_else(|| anyhow::anyhow!("Provider {name} not found"))?;
    let model_id = if requested_model.is_empty() {
        codetether_provider_policy::default_model_for_provider(&name)
    } else {
        requested_model
    };
    let step = host.build(provider.clone(), &name, &model_id, session);
    Ok(ModelState::new(name, model_id, provider, step))
}
