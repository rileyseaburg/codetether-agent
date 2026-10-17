//! Public context-derivation entry point and final state header.
use super::*;

/// Derive an ephemeral [`DerivedContext`] under a chosen [`DerivePolicy`].
///
/// Generalisation of [`derive_context`] that accepts a policy selector.
/// [`DerivePolicy::Legacy`] delegates back to `derive_context` so the
/// two signatures co-exist without behaviour drift during rollout.
///
/// # Arguments
///
/// Same as [`derive_context`], plus:
///
/// * `policy` — Which derivation strategy to run. See [`DerivePolicy`].
/// * `force_keep_last` — When `Some(n)`, bypass policy selection and fall
///   back to the legacy keep-last derivation used for prompt-too-long
///   recovery.
///
/// # Errors
///
/// Propagates any error from the underlying pipeline.
pub async fn derive_with_policy(
    session: &Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    policy: DerivePolicy,
    force_keep_last: Option<usize>,
) -> Result<DerivedContext> {
    let input = DispatchInputs {
        session,
        provider,
        model,
        system_prompt,
        tools,
        event_tx,
        force_keep_last,
    };
    let mut derived = execute::run(input, select(policy, force_keep_last)).await?;
    state_header::prepend_state_header(session, &mut derived.messages);
    Ok(derived)
}
