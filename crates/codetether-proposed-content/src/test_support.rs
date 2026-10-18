//! File fixture for proposed-content regression cases.

pub(super) fn fixture() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("src/lib.rs");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "fn old() {}\nfn keep() {}\n").unwrap();
    (dir, path)
}
