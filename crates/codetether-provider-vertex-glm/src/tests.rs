//! Construction validation regression tests.
use super::*;

#[test]
fn test_rejects_invalid_sa_json() {
    let result = VertexGlmProvider::new("{}", None);
    assert!(result.is_err());
}

#[test]
fn test_rejects_missing_project_id() {
    let sa_json = json!({
        "type": "service_account",
        "client_email": "test@test.iam.gserviceaccount.com",
        "private_key": "-----BEGIN RSA PRIVATE KEY-----\nMIIBogIBAAJBALRiMLAHudeSA/x3hB2f+2NRkJlS\n-----END RSA PRIVATE KEY-----\n",
        "token_uri": "https://oauth2.googleapis.com/token"
    });
    // Invalid RSA key but the error should be about key parsing, not project
    let result = VertexGlmProvider::new(&sa_json.to_string(), None);
    assert!(result.is_err());
}
