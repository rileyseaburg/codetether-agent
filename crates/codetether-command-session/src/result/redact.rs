//! Secret replacement for model-visible process output.

pub(super) fn output(output: &str, secrets: &[String]) -> String {
    let mut output = output.to_string();
    for secret in secrets {
        if !secret.is_empty() {
            output = output.replace(secret, "[REDACTED]");
        }
    }
    output
}
