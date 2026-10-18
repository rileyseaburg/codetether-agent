//! Apply the shared prior-context restriction to the application registry.
use crate::{Application, ToolRegistry};
/// Remove each tool that can consult prior session context.
/// # Arguments
/// `registry` is filtered in place using the shared prior-context tool set.
/// # Returns
/// Completes after removing all prior-context tool identifiers.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_application_registry::Application>(registry: &mut codetether_application_registry::ToolRegistry<H>) {
/// codetether_application_registry::session_tools::remove_prior_context(registry);
/// assert!(registry.get("session_recall").is_none());
/// # }
/// ```
pub fn remove_prior_context<H: Application>(registry: &mut ToolRegistry<H>) {
    codetether_prior_context_access::remove_tools(|name| {
        registry.unregister(name);
    });
}
