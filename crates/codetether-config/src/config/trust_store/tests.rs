use super::store::ProjectTrustStore;
use super::workspace::{canonical_workspace_path, workspace_key};
use sha2::{Digest, Sha256};
use tempfile::{TempDir, tempdir};

#[test]
fn workspace_key_hashes_canonical_workspace_path() {
    let (_tmp, repo, nested) = temp_repo();
    let canonical = canonical_workspace_path(&nested).expect("canonical workspace");
    let mut hasher = Sha256::new();
    hasher.update(canonical.to_string_lossy().as_bytes());
    assert_eq!(canonical, std::fs::canonicalize(repo).expect("repo path"));
    assert_eq!(workspace_key(&canonical), hex::encode(hasher.finalize()));
}

#[test]
fn trust_and_untrust_toggle_workspace_record() {
    let (_repo_tmp, _repo, nested) = temp_repo();
    let data = tempdir().expect("data dir");
    let store = ProjectTrustStore::with_base(data.path(), &nested).expect("store");
    assert!(!store.is_trusted());
    store.trust().expect("trust workspace");
    assert!(store.is_trusted());
    assert!(store.record_path().starts_with(data.path()));
    store.untrust().expect("untrust workspace");
    assert!(!store.is_trusted());
    store.untrust().expect("untrust stays idempotent");
}

fn temp_repo() -> (TempDir, std::path::PathBuf, std::path::PathBuf) {
    let tmp = tempdir().expect("tempdir");
    let repo = tmp.path().join("repo");
    std::fs::create_dir_all(repo.join(".git")).expect("create git marker");
    let nested = repo.join("src").join("nested");
    std::fs::create_dir_all(&nested).expect("create nested");
    (tmp, repo, nested)
}
