//! Public native specialization of the shared Ralph runtime.
/// Ralph run state and service integration with the native application bindings.
/// # Examples
/// ```rust,no_run
/// # async fn example(provider: std::sync::Arc<dyn codetether_agent::provider::Provider>) -> anyhow::Result<()> {
/// let run = codetether_agent::ralph::RalphLoop::new("prd.json".into(), provider, "model".into(), Default::default()).await?;
/// assert_eq!(run.status().status, codetether_agent::ralph::RalphStatus::Pending);
/// # Ok(()) }
/// ```
pub type RalphLoop = codetether_ralph_runtime::RalphLoop<super::runtime_host::Native>;
pub use codetether_ralph_runtime::create_prd_template;
