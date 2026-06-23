//! Tests for SSO role-credential parsing and conversion.

#[cfg(test)]
mod tests {
    use super::super::role_creds_types::{RoleCredsEnvelope, to_exported};

    #[test]
    fn parses_and_converts_role_credentials() {
        let json = r#"{
            "roleCredentials": {
                "accessKeyId": "AKIAEXAMPLE",
                "secretAccessKey": "secret",
                "sessionToken": "token",
                "expiration": 1700000000000
            }
        }"#;
        let env: RoleCredsEnvelope = serde_json::from_str(json).expect("valid JSON");
        let exported = to_exported(env.role_credentials);
        assert_eq!(exported.creds.access_key_id, "AKIAEXAMPLE");
        assert_eq!(exported.creds.secret_access_key, "secret");
        assert_eq!(exported.creds.session_token.as_deref(), Some("token"));
        assert!(exported.expiration.is_some());
    }
}
