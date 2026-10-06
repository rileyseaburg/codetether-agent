//! Identity selection for loaded native runtimes and enclosing providers.

pub(super) fn system_prompt(caller: &str, provider: &str, loaded_model: &str) -> String {
    crate::provider::metrics::identity::system_prompt(caller, provider, loaded_model)
}

#[cfg(test)]
#[path = "identity_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "identity_provider_switch_tests.rs"]
mod provider_switch_tests;
