//! Retained synthetic project instructions for scope-isolation tests.
use std::path::PathBuf;
pub fn repository() -> PathBuf {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("chat-scope-{}-{stamp}", std::process::id()));
    std::fs::create_dir_all(path.join(".git")).unwrap();
    std::fs::write(path.join("AGENTS.md"), "PROJECT_ONLY_SHOULD_NOT_LEAK").unwrap();
    eprintln!("Retained mocked-local scope fixture: {}", path.display());
    path
}
