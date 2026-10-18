use super::{Host, OutputCallback, PolicySessionResult, Registry};
use crate::execution_policy::{AutoApprove, model_selection as session_model};
use codetether_provider_core::{ContentPart, Message, Role};
/// Prepare and execute a standard worker session.
/// # Arguments
/// `host` supplies native services; remaining inputs preserve the task prompt,
/// approval policy, model tier, metadata and output observer.
/// # Returns
/// Final text, session identity and step-budget outcome.
/// # Errors
/// Propagates provider, title-generation or session execution failures.
/// # Examples
/// ```no_run
/// # async fn run(host: &mut impl codetether_a2a_worker_core::session_protocol::policy::Host) -> anyhow::Result<()> {
/// let _result = codetether_a2a_worker_core::session_protocol::policy::execute(
///     host, "Inspect the project", codetether_a2a_worker_core::execution_policy::AutoApprove::Safe,
///     None, &serde_json::Map::new(), None,
/// ).await?;
/// # Ok(()) }
/// ```
pub async fn execute(
    host: &mut impl Host,
    prompt: &str,
    auto_approve: AutoApprove,
    model_tier: Option<&str>,
    task_metadata: &serde_json::Map<String, serde_json::Value>,
    output_callback: OutputCallback,
) -> anyhow::Result<PolicySessionResult> {
    let registry = host.load().await?;
    let providers = registry.list();
    session_model::ensure_providers(&providers)?;
    let selection = session_model::select(host.model(), &providers, model_tier)?;
    let provider = registry
        .get(&selection.provider)
        .ok_or_else(|| anyhow::anyhow!("Provider {} not found", selection.provider))?;
    host.append(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: prompt.to_string(),
        }],
    });
    if host.needs_title() {
        host.title(&registry).await?;
    }
    super::run_selected::run(
        host,
        provider,
        selection,
        auto_approve,
        model_tier,
        task_metadata,
        output_callback,
    )
    .await
}
