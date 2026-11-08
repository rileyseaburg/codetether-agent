//! Check OAuth state, issuer and authorization result without exposing secrets.
use super::{Callback, request::parameters};
use anyhow::{Result, anyhow, ensure};
use subtle::ConstantTimeEq;

pub(super) fn parse(request: &str, authority: &str, state: &str) -> Result<Callback> {
    let mut values = parameters(request, authority)?;
    let supplied = values
        .get("state")
        .ok_or_else(|| anyhow!("Missing callback state"))?;
    ensure!(
        bool::from(supplied.as_bytes().ct_eq(state.as_bytes())),
        "Invalid callback state"
    );
    if let Some(issuer) = values.get("iss") {
        ensure!(
            issuer == crate::discovery::ISSUER,
            "Invalid callback issuer"
        );
    }
    if values.contains_key("error") {
        ensure!(!values.contains_key("code"), "Ambiguous callback response");
        return Ok(Callback::Denied);
    }
    let code = values
        .remove("code")
        .ok_or_else(|| anyhow!("Missing authorization code"))?;
    ensure!(
        !code.is_empty() && code.len() <= 4096 && !code.chars().any(char::is_control),
        "Invalid authorization code"
    );
    Ok(Callback::Code(code))
}
