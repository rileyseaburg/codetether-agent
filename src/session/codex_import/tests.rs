use super::parse::parse_codex_session_from_path;
use super::persist::persist_imported_session;
use super::test_support::write_codex_fixture;
use super::{discover::discover_codex_sessions_in, info::PersistOutcome};
use crate::provider::{ContentPart, Role};
use tempfile::tempdir;

#[test]
fn parses_codex_session_into_native_session() {
    let temp = tempdir().expect("tempdir");
    let workspace = temp.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("workspace");
    let path = write_codex_fixture(temp.path(), &workspace);
    let session = parse_codex_session_from_path(&path, Some("Imported from Codex")).expect("parse");
    assert_eq!(session.id, "019d2acd-8b3f-70e0-b019-854d52272660");
    assert_eq!(session.metadata.model.as_deref(), Some("gpt-5.4"));
    assert_eq!(session.usage.total_tokens, 15);
    assert_eq!(session.messages.len(), 5);
    assert!(matches!(session.messages[0].role, Role::User));
    assert!(matches!(
        session.messages[1].content[0],
        ContentPart::Thinking { .. }
    ));
    assert!(matches!(
        session.messages[2].content[0],
        ContentPart::ToolCall { .. }
    ));
}

#[test]
fn discovers_codex_sessions_for_matching_workspace() {
    let temp = tempdir().expect("tempdir");
    let workspace = temp.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("workspace");
    write_codex_fixture(temp.path(), &workspace);
    let codex_home = temp.path().join(".codex");
    let sessions = discover_codex_sessions_in(Some(&codex_home), &workspace).expect("discover");
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].message_count, 5);
}

#[tokio::test]
async fn persists_imported_session_only_when_newer() {
    let temp = tempdir().expect("tempdir");
    let workspace = temp.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("workspace");
    let path = write_codex_fixture(temp.path(), &workspace);
    let session = parse_codex_session_from_path(&path, Some("Imported from Codex")).expect("parse");
    let data_dir = temp.path().join("data/sessions");
    let first = persist_imported_session(session.clone(), &data_dir)
        .await
        .expect("persist");
    let second = persist_imported_session(session, &data_dir)
        .await
        .expect("persist");
    assert!(matches!(first, PersistOutcome::Saved));
    assert!(matches!(second, PersistOutcome::Unchanged));
}
