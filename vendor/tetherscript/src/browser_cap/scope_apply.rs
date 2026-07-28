//! Apply browser narrowing fields.

use std::collections::HashSet;

use crate::value::Value;

use super::authority::BrowserAuthority;

pub(crate) fn origins(
    auth: &BrowserAuthority,
    next: &mut BrowserAuthority,
    value: &Value,
) -> Result<(), String> {
    let requested: Vec<String> = super::scope_list::string_list(value, "browser.narrow origins")?
        .into_iter()
        .map(super::origin::normalize_origin)
        .collect();
    if !auth.allowed_origins.is_empty()
        && !requested.iter().all(|r| auth.allowed_origins.contains(r))
    {
        return Err("browser.narrow: requested origins are not a subset".into());
    }
    next.allowed_origins = requested;
    Ok(())
}

pub(crate) fn scopes(
    auth: &BrowserAuthority,
    next: &mut BrowserAuthority,
    value: &Value,
) -> Result<(), String> {
    let requested: HashSet<String> =
        super::scope_list::string_list(value, "browser.narrow scopes")?
            .into_iter()
            .collect();
    if !requested.iter().all(|s| auth.allowed_scopes.contains(s)) {
        return Err("browser.narrow: requested scopes are not a subset".into());
    }
    next.allowed_scopes = requested;
    Ok(())
}
