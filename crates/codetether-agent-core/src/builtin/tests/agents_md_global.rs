//! Focused tests for Codex-compatible global instruction discovery.

use super::super::agents_md::load_all_agents_md_with_codex_home;
use tempfile::tempdir;

#[test]
fn global_codex_home_agents_loads_before_project_agents() {
    let tmp = tempdir().expect("tempdir");
    let codex_home = tmp.path().join("codex");
    let repo = tmp.path().join("repo");
    std::fs::create_dir_all(&codex_home).expect("create codex home");
    std::fs::create_dir_all(repo.join(".git")).expect("create repo");
    let global = codex_home.join("AGENTS.md");
    let project = repo.join("AGENTS.md");
    std::fs::write(&global, "global").expect("write global");
    std::fs::write(&project, "project").expect("write project");

    let loaded = load_all_agents_md_with_codex_home(&repo, 32 * 1024, &codex_home);

    assert_eq!(loaded[0], ("global".to_string(), global));
    assert_eq!(loaded[1], ("project".to_string(), project));
}
