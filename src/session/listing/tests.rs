//! Tests for the append-only session listing index.

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

use chrono::Utc;
use tempfile::TempDir;

use super::directory::sessions_dir;
use super::directory_query::list_indexed;
use super::index_file_compact::should_compact;
use super::index_file_io;
use super::summary::SessionSummary;

static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Point `CODETETHER_DATA_DIR` at `tmp` and resolve the sessions dir.
/// The returned guard serialises env mutation across tests; hold it for
/// the duration of the test body.
fn enter_data_dir(tmp: &TempDir) -> (MutexGuard<'static, ()>, PathBuf) {
    let guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    std::fs::create_dir_all(tmp.path().join("sessions")).unwrap();
    // SAFETY: serialised by ENV_LOCK above.
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", tmp.path()) };
    (guard, sessions_dir().unwrap())
}

fn make_summary(id: &str, title: Option<&str>, updated_offset_secs: i64) -> SessionSummary {
    let now = Utc::now() + chrono::Duration::seconds(updated_offset_secs);
    SessionSummary {
        id: id.to_string(),
        title: title.map(String::from),
        created_at: now,
        updated_at: now,
        message_count: 0,
        agent: "test".to_string(),
        directory: None,
    }
}

fn write_session_json(sessions: &std::path::Path, id: &str, workspace_label: &str, count: usize) {
    let payload = serde_json::json!({
        "id": id,
        "title": format!("title-{id}"),
        "created_at": "2026-01-01T00:00:00Z",
        "updated_at": "2026-01-02T00:00:00Z",
        "metadata": {
            "directory": format!("/tmp/{workspace_label}"),
            "shared": false,
            "share_url": null
        },
        "agent": "test",
        "messages": (0..count).map(|i| serde_json::json!({"i": i})).collect::<Vec<_>>()
    });
    let path = sessions.join(format!("{id}.json"));
    std::fs::write(path, serde_json::to_vec(&payload).unwrap()).unwrap();
}

#[tokio::test]
async fn index_round_trip_preserves_summaries() {
    let tmp = tempfile::tempdir().unwrap();
    let (_guard, sessions) = enter_data_dir(&tmp);
    let a = make_summary("aaa", Some("alpha"), 0);
    let b = make_summary("bbb", Some("beta"), 10);
    let p = sessions.join("index.jsonl");
    index_file_io::append_sync(&p, &a).unwrap();
    index_file_io::append_sync(&p, &b).unwrap();
    let (map, lines) = index_file_io::read_sync(&p).unwrap();
    assert_eq!(map.len(), 2);
    assert_eq!(lines, 2);
    assert_eq!(map.get("aaa").unwrap().title.as_deref(), Some("alpha"));
    assert_eq!(map.get("bbb").unwrap().title.as_deref(), Some("beta"));
}

#[tokio::test]
async fn last_line_wins_per_id() {
    let tmp = tempfile::tempdir().unwrap();
    let (_guard, sessions) = enter_data_dir(&tmp);
    let p = sessions.join("index.jsonl");
    let old = make_summary("same", Some("old"), 0);
    let new = make_summary("same", Some("new"), 5);
    index_file_io::append_sync(&p, &old).unwrap();
    index_file_io::append_sync(&p, &new).unwrap();
    let (map, lines) = index_file_io::read_sync(&p).unwrap();
    assert_eq!(map.len(), 1);
    assert_eq!(lines, 2);
    assert_eq!(map.get("same").unwrap().title.as_deref(), Some("new"));
}

#[tokio::test]
async fn rebuild_from_scan_when_index_missing() {
    let tmp = tempfile::tempdir().unwrap();
    let (_guard, sessions) = enter_data_dir(&tmp);
    write_session_json(&sessions, "s1", "workspace-a", 3);
    write_session_json(&sessions, "s2", "workspace-b", 1);
    let summaries = list_indexed(sessions.clone(), None).await.unwrap();
    assert_eq!(summaries.len(), 2);
    assert!(sessions.join("index.jsonl").exists());
}

#[tokio::test]
async fn workspace_scoped_rebuild_keeps_full_index() {
    let tmp = tempfile::tempdir().unwrap();
    let (_guard, sessions) = enter_data_dir(&tmp);
    write_session_json(&sessions, "mine", "workspace-a", 1);
    write_session_json(&sessions, "other", "workspace-b", 1);
    let scoped = list_indexed(sessions.clone(), Some(PathBuf::from("/tmp/workspace-a")))
        .await
        .unwrap();
    assert_eq!(scoped.len(), 1);
    assert_eq!(scoped[0].id, "mine");
    // The rebuild must not drop the other workspace's session from the index.
    let (map, _) = index_file_io::read_sync(&sessions.join("index.jsonl")).unwrap();
    assert!(map.contains_key("other"));
}

#[tokio::test]
async fn disk_only_session_still_appears_in_listing() {
    let tmp = tempfile::tempdir().unwrap();
    let (_guard, sessions) = enter_data_dir(&tmp);
    let stale = make_summary("stale", Some("stale"), 0);
    let p = sessions.join("index.jsonl");
    index_file_io::append_sync(&p, &stale).unwrap();
    write_session_json(&sessions, "fresh", "ws", 2);
    write_session_json(&sessions, "stale", "ws", 1);
    let summaries = list_indexed(sessions.clone(), None).await.unwrap();
    let ids: Vec<&str> = summaries.iter().map(|s| s.id.as_str()).collect();
    assert!(ids.contains(&"fresh"));
    // No duplicates: each id appears exactly once.
    assert_eq!(summaries.len(), 2);
}

#[tokio::test]
async fn entry_for_deleted_file_is_dropped() {
    let tmp = tempfile::tempdir().unwrap();
    let (_guard, sessions) = enter_data_dir(&tmp);
    write_session_json(&sessions, "live", "ws", 1);
    write_session_json(&sessions, "doomed", "ws", 2);
    let live = make_summary("live", None, 0);
    let doomed = make_summary("doomed", None, 0);
    let p = sessions.join("index.jsonl");
    index_file_io::append_sync(&p, &live).unwrap();
    index_file_io::append_sync(&p, &doomed).unwrap();
    std::fs::remove_file(sessions.join("doomed.json")).unwrap();
    let summaries = list_indexed(sessions.clone(), None).await.unwrap();
    let ids: Vec<&str> = summaries.iter().map(|s| s.id.as_str()).collect();
    assert!(ids.contains(&"live"));
    assert!(!ids.contains(&"doomed"));
}

#[test]
fn should_compact_triggers_at_four_x() {
    assert!(!should_compact(0, 0));
    assert!(!should_compact(3, 1));
    assert!(should_compact(4, 1));
    assert!(should_compact(100, 10));
}
