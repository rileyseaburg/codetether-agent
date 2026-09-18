//! Profile persistence regression tests.
use super::*;
#[test]
fn saved_vault_profile_round_trips_without_falling_back_after_logout() {
    let _lock = crate::approval::test_env::lock_env();
    let dir = tempfile::tempdir().unwrap();
    let _directory = super::test_env::Directory::set(dir.path());
    assert!(load().unwrap().is_none());
    let mut profile = Profile {
        address: "https://vault.example".into(),
        token: Some("fixture-token".into()),
    };
    save(&profile).unwrap();
    assert_eq!(
        load().unwrap().unwrap().token.as_deref(),
        Some("fixture-token")
    );
    profile.token = None;
    save(&profile).unwrap();
    assert!(load().unwrap().unwrap().token.is_none());
}
