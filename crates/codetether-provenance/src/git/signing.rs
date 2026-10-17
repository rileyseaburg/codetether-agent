//! Git commit signing policy selected from runtime configuration.
/// Return Git commit arguments selected by the signing environment policy.
///
/// # Returns
/// A static argument list including either signing or its explicit disable flag.
///
/// # Examples
/// ```
/// use codetether_provenance::commit_args_with_signature_policy;
/// assert_eq!(commit_args_with_signature_policy()[0], "commit");
/// ```
pub fn commit_args_with_signature_policy() -> &'static [&'static str] {
    if codetether_commit_signing_enabled() {
        &["commit", "-S", "-m"]
    } else {
        &["commit", "--no-gpg-sign", "-m"]
    }
}

fn codetether_commit_signing_enabled() -> bool {
    matches!(
        std::env::var("CODETETHER_GIT_COMMIT_SIGN"),
        Ok(value) if !is_falsey(&value)
    )
}

fn is_falsey(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "" | "0" | "false" | "no" | "off"
    )
}

#[cfg(test)]
mod tests {
    use super::is_falsey;

    #[test]
    fn treats_common_disabled_values_as_falsey() {
        for value in ["", "0", "false", "no", "off"] {
            assert!(is_falsey(value));
        }
    }

    #[test]
    fn treats_enabled_values_as_truthy() {
        assert!(!is_falsey("1"));
        assert!(!is_falsey("true"));
    }
}
