//! Refresh routing metadata for transports carrying a single system string.

/// Remove only leading, dedicated harness blocks, not embedded caller content.
pub(crate) fn caller_prompt(mut system: &str) -> &str {
    while system.starts_with(super::START) {
        let Some(end) = super::block::identity_len(system) else {
            break;
        };
        let rest = &system[end..];
        if rest.is_empty() {
            return rest;
        }
        let Some(rest) = rest.strip_prefix("\n\n") else {
            break;
        };
        system = rest;
    }
    system
}

/// Prepend current metadata while retaining the caller's prompt verbatim.
pub(crate) fn system_prompt(system: &str, provider: &str, model: &str) -> String {
    let identity = super::prompt(provider, model);
    let caller = caller_prompt(system);
    if caller.is_empty() {
        identity
    } else {
        format!("{identity}\n\n{caller}")
    }
}

#[cfg(test)]
#[path = "system_tests.rs"]
mod tests;
