//! Filter registry entries through the same prior-context tool set.
/// Whether a tool remains available under the supplied permission decision.
/// # Arguments
/// Receives effective permission and the requested tool name.
/// # Returns
/// The effective permission or detection decision.
/// # Examples
/// ```
/// assert!(!codetether_prior_context_access::available(false, "session_recall"));
/// ```
pub fn available(allowed: bool, name: &str) -> bool {
    allowed || !super::call::PRIOR_CONTEXT_TOOLS.contains(&name)
}
/// Remove each prior-context tool using a native registry callback.
/// # Arguments
/// Receives a callback that unregisters one tool by name.
/// # Returns
/// Updates the supplied state in place.
/// # Examples
/// ```
/// let mut removed = Vec::new();
/// codetether_prior_context_access::remove_tools(|name| removed.push(name.to_owned()));
/// assert_eq!(removed.len(), 4);
/// ```
pub fn remove_tools(mut unregister: impl FnMut(&str)) {
    for name in super::call::PRIOR_CONTEXT_TOOLS {
        unregister(name);
    }
}
