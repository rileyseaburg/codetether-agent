//! Rejection messages keep Vault's reason only when it leaks nothing.

use super::rejection::message;

#[test]
fn surfaces_vault_reason() {
    let body = r#"{"errors":["invalid audience (aud) claim"]}"#;
    assert_eq!(
        message(400, body, &["eyJsecret"]),
        "Authentication request rejected (HTTP 400): invalid audience (aud) claim"
    );
}

#[test]
fn drops_reasons_that_echo_secrets_or_tokens() {
    let echo = r#"{"errors":["fixture-token"]}"#;
    assert_eq!(
        message(403, echo, &["fixture-token"]),
        "Authentication request rejected (HTTP 403)"
    );
    let token = r#"{"errors":["bad token hvs.CAESIabc"]}"#;
    assert_eq!(
        message(403, token, &[]),
        "Authentication request rejected (HTTP 403)"
    );
    assert_eq!(
        message(502, "<html>", &[]),
        "Authentication request rejected (HTTP 502)"
    );
}
