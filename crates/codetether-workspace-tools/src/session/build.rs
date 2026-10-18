use super::{Factory, autonomous, batch, file_uri};
use crate::Registry;
use codetether_provider_core::Provider;
use std::{path::Path, sync::Arc};
/// Build a session catalog whose shell and LSP tools target the workspace.
/// # Arguments
/// * `provider`, `model` — Completion backend and selected model.
/// * `cwd` — Assigned session working directory.
/// * `prior_context_allowed` — Whether historical-context tools remain available.
/// * `autonomous` — Whether to remove tools requiring live user interaction.
/// # Returns
/// The configured shared registry with a weakly bound batch tool.
/// # Examples
/// ```no_run
/// # fn build<H: codetether_workspace_tools::session::Factory>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let _registry = codetether_workspace_tools::session::registry_for_cwd::<H>(provider, "model", std::path::Path::new("."), false, true);
/// # }
/// ```
pub fn registry_for_cwd<H: Factory>(
    provider: Arc<dyn Provider>,
    model: &str,
    cwd: &Path,
    prior_context_allowed: bool,
    autonomous: bool,
) -> Arc<H::Registry> {
    let mut registry = H::provider(Arc::clone(&provider), model.to_string());
    H::application_mcp(&mut registry);
    H::commands(&mut registry, cwd);
    registry.register(H::lsp(file_uri::build(cwd)));
    H::search(&mut registry, provider);
    if !prior_context_allowed {
        H::remove_prior_context(&mut registry);
    }
    if autonomous {
        autonomous::restrict(&mut registry);
    }
    batch::with_batch::<H>(registry)
}
