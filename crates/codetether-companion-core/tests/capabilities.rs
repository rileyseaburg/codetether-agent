mod common;
use codetether_companion_core::{Error, OwnerCredential, Registry};
const OWNER: &str = "synthetic-owner-credential-not-a-real-secret";

#[test]
fn device_capabilities_are_scoped_and_distinct_from_owner_credentials() {
    let mut registry = Registry::default();
    let first = registry.create(common::input(), common::NOW).unwrap();
    let second = registry.create(common::input(), common::NOW).unwrap();
    let pair = registry.pair(&first.code, common::NOW).unwrap();
    let auth = format!("Bearer {}", pair.device_token);
    let owner = OwnerCredential::new(OWNER).unwrap();
    assert!(owner.authorize(Some(&auth)).is_err());
    let result = registry.authorize_device(&first.id, Some(&auth), common::NOW);
    assert!(result.is_ok());
    let result = registry.authorize_device(&second.id, Some(&auth), common::NOW);
    assert!(result.is_err());
    let owner_auth = format!("Bearer {OWNER}");
    let result = registry.authorize_device(&first.id, Some(&owner_auth), common::NOW);
    assert!(result.is_err());
    assert_eq!(pair.device_token.len(), 43);
    assert_eq!(pair.expires_at, first.expires_at);
    assert_eq!(pair.interval_seconds, 30);
    assert!(
        pair.device_token
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"_-".contains(&c))
    );
    for header in [
        None,
        Some("Bearer "),
        Some("Bearer x/y"),
        Some("Bearer x\r\n"),
    ] {
        let result = registry.authorize_device(&first.id, header, common::NOW);
        assert_eq!(result, Err(Error::AuthenticationRequired));
    }
    let other = registry.pair(&second.code, common::NOW).unwrap();
    assert_ne!(pair.device_token, other.device_token);
}
