//! Bind configured or process-derived worker identity.
/// Resolve and bind the worker name using the existing provenance registry.
/// # Arguments
/// `configured` is the optional explicit worker name.
/// # Returns
/// The bound runtime identity, falling back to the process-derived name.
/// # Examples
/// ```no_run
/// let name = codetether_a2a_worker_core::initialization::resolve_name(None);
/// assert!(!name.is_empty());
/// ```

pub fn resolve_name(configured: Option<&str>) -> String {
    match configured {
        Some(name) => codetether_provenance::bind_runtime_agent_identity(name),
        None => codetether_provenance::ensure_runtime_agent_identity(&format!(
            "codetether-{}",
            std::process::id()
        )),
    }
}
