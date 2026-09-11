//! Task lists are keyed by session id, never merged across a shared checkout.

use serde_json::json;

use super::{ROOT_FILE, load, resolve, save};
use crate::approval::test_env::lock_env;
use crate::tool::todo::{Priority, TodoItem, TodoStatus};

#[test]
fn session_id_selects_a_per_session_file() {
    let _lock = lock_env();
    let temp = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", temp.path()) };
    let path = resolve(temp.path(), &json!({"__ct_session_id": "abc-1"}));
    assert_eq!(path, temp.path().join("todos/abc-1.json"));
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
}

#[test]
fn unsafe_session_id_is_ignored() {
    let root = std::path::Path::new("/repo");
    let path = resolve(root, &json!({"__ct_session_id": "../x"}));
    assert_eq!(path, root.join(ROOT_FILE));
}

#[test]
fn two_sessions_in_one_checkout_keep_separate_lists() {
    let _lock = lock_env();
    let temp = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", temp.path()) };
    let first = json!({"__ct_session_id": "mux-a"});
    let second = json!({"__ct_session_id": "mux-b"});
    let item = TodoItem {
        id: "t1".into(),
        content: "only for mux-a".into(),
        status: TodoStatus::Pending,
        priority: Priority::Medium,
        created_at: None,
    };
    save(temp.path(), &first, &[item]).unwrap();
    assert_eq!(load(temp.path(), &first).unwrap().len(), 1);
    assert!(load(temp.path(), &second).unwrap().is_empty());
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
}
