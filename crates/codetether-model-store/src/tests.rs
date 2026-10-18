//! Tests for the local model store.

use super::ModelStore;

#[test]
fn round_trips_models_through_disk() {
    let tmp = tempfile::tempdir().unwrap();
    let store = ModelStore::with_base(tmp.path());

    assert!(store.load().is_empty());

    let models = vec!["openai/gpt-4o".to_string(), "zai/glm-5.1".to_string()];
    store.store(&models);

    assert_eq!(store.load(), models);
}

#[test]
fn load_returns_empty_when_file_missing() {
    let tmp = tempfile::tempdir().unwrap();
    let store = ModelStore::with_base(tmp.path());
    assert!(store.load().is_empty());
}

#[test]
fn load_returns_empty_on_corrupt_json() {
    let tmp = tempfile::tempdir().unwrap();
    let store = ModelStore::with_base(tmp.path());
    std::fs::create_dir_all(store.path().parent().unwrap()).unwrap();
    std::fs::write(store.path(), b"not json").unwrap();
    assert!(store.load().is_empty());
}
