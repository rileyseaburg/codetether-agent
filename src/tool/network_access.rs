//! Effective network permission shared by command tools and approval scope.

#[path = "network_authorize.rs"]
mod authorize;

pub(crate) use authorize::{args, guard, invocation};

#[path = "network_access_helpers.rs"]
mod helpers;
pub(crate) use helpers::{no_redirect_client, no_redirect_client_with_timeout};
#[path = "network_access_trusted.rs"]
mod trusted;
pub(crate) use trusted::{bind_trusted, value as trusted_value, workspace as trusted_workspace};
#[path = "network_access_env.rs"]
mod env;
pub(crate) use env::allowed;

#[cfg(test)]
#[path = "network_test_env.rs"]
pub(crate) mod test_env;

use serde_json::{Value, json};

pub(crate) const FIELD: &str = "__ct_effective_network_allowed";

pub(crate) fn allowed_for(args: &Value) -> bool {
    trusted::value(args).unwrap_or_else(allowed)
}

pub(crate) fn bind(args: &mut Value, value: bool) {
    if let Some(map) = args.as_object_mut() {
        map.insert(FIELD.into(), json!(value));
    }
}

pub(crate) fn require(tool: &str, args: &Value) -> anyhow::Result<()> {
    if !allowed_for(args) {
        anyhow::bail!("network access is disabled for {tool}");
    }
    Ok(())
}

#[cfg(test)]
#[path = "network_access_tests.rs"]
mod tests;
#[cfg(test)]
#[path = "network_alias_tests.rs"]
mod alias_tests;