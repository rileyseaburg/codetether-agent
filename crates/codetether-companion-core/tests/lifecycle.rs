mod common;
use codetether_companion_core::{Error, Registry};

#[test]
fn receipts_pairing_and_revocation() {
    let mut registry = Registry::default();
    let receipt = registry.create(common::input(), common::NOW).unwrap();
    assert_eq!(receipt.pair_expires_at, "2026-01-01T00:05:00.000Z");
    assert_eq!(receipt.expires_at, "2026-01-01T01:00:00.000Z");
    assert_eq!(receipt.code.len(), 12);
    assert!(receipt.code.bytes().all(|c| c.is_ascii_hexdigit()));
    assert_eq!(receipt.code, receipt.code.to_uppercase());
    let session = registry.session(&receipt.id, common::NOW).unwrap();
    assert!(!session.is_paired());
    assert_eq!(session.input().unwrap().prompt, "Describe the screen");
    let code = format!(
        "\u{feff}{}-{}\u{00a0}",
        receipt.code[..6].to_lowercase(),
        &receipt.code[6..]
    );
    let pair = registry.pair(&code, common::NOW).unwrap();
    let session = registry.session(&receipt.id, common::NOW).unwrap();
    assert!(session.is_paired());
    assert_eq!(
        registry.pair(&receipt.code, common::NOW).err(),
        Some(Error::Pairing)
    );
    registry.stop(&receipt.id, common::NOW).unwrap();
    let auth = format!("Bearer {}", pair.device_token);
    let result = registry.authorize_device(&receipt.id, Some(&auth), common::NOW);
    assert_eq!(result, Err(Error::Ended));
    assert_eq!(
        registry.session(&receipt.id, common::NOW).err(),
        Some(Error::Ended)
    );
    registry.sweep(common::NOW);
    assert!(registry.is_empty());
    assert_eq!(
        registry.session(&receipt.id, common::NOW).err(),
        Some(Error::NotFound)
    );
}
