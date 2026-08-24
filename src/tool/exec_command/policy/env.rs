//! Environment overrides shared with the legacy bash tool.

pub(super) fn truthy(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| value_truthy(&value))
}

pub(crate) fn network_allowed() -> bool {
    crate::tool::network_access::allowed()
}

fn value_truthy(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

pub(super) fn is_false(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| {
        matches!(
            value.to_ascii_lowercase().as_str(),
            "0" | "false" | "no" | "off"
        )
    })
}

#[cfg(test)]
mod tests {
    use super::value_truthy;

    #[test]
    fn truthy_values_are_case_insensitive() {
        for value in ["1", "true", "TRUE", "Yes", "ON"] {
            assert!(value_truthy(value));
        }
    }

    #[test]
    fn false_and_unknown_values_are_rejected() {
        for value in ["0", "false", "no", "off", "unknown", ""] {
            assert!(!value_truthy(value));
        }
    }
}