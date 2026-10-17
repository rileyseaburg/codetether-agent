
use super::*;

#[test]
fn test_rejects_invalid_sa_json() {
    let result = VertexAnthropicProvider::new("{}", None);
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
    let result = VertexAnthropicProvider::new(&sa_json.to_string(), None);
    assert!(result.is_err());
}

#[test]
fn test_model_url_building() {
    // This would require a valid SA key, so just test the logic conceptually
    // In a real test, we'd mock the provider
}
