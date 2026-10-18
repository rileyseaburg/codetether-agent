//! Read configured strings and boolean aliases from provider secrets.

pub(super) fn vault_str(s: &codetether_secrets::ProviderSecrets, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|k| {
        s.extra
            .get(*k)
            .and_then(|v| v.as_str())
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
    })
}

pub(super) fn vault_bool_val(
    s: &codetether_secrets::ProviderSecrets,
    keys: &[&str],
) -> Option<bool> {
    keys.iter().find_map(|k| {
        s.extra
            .get(*k)
            .and_then(|v| v.as_str())
            .and_then(|v| match v {
                "true" | "1" | "yes" => Some(true),
                "false" | "0" | "no" => Some(false),
                _ => None,
            })
    })
}
