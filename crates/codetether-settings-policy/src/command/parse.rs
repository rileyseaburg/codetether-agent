//! Shared on/off/toggle/status command interpretation.
pub(super) fn desired(rest: &str, current: bool) -> Result<Option<bool>, ()> {
    match rest.trim().to_ascii_lowercase().as_str() {
        "" | "toggle" => Ok(Some(!current)),
        "status" => Ok(None),
        "on" | "true" | "yes" | "enable" | "enabled" => Ok(Some(true)),
        "off" | "false" | "no" | "disable" | "disabled" => Ok(Some(false)),
        _ => Err(()),
    }
}
