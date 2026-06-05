use serde_json::json;

use super::scan::scan;

#[tokio::test]
async fn scan_counts_messages_and_filters_workspace() {
    let temp = tempfile::tempdir().unwrap();
    let sessions = temp.path().join("sessions");
    let workspace = temp.path().join("workspace");
    let other = temp.path().join("other");
    std::fs::create_dir_all(&sessions).unwrap();
    std::fs::create_dir_all(&workspace).unwrap();
    std::fs::create_dir_all(&other).unwrap();

    write_session(&sessions, "keep", &workspace, 3);
    write_session(&sessions, "skip", &other, 2);

    let summaries = scan(sessions, Some(workspace)).await.unwrap();
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].id, "keep");
    assert_eq!(summaries[0].message_count, 3);
}

fn write_session(sessions: &std::path::Path, id: &str, dir: &std::path::Path, count: usize) {
    let large = "x".repeat(1024 * 1024);
    let messages = (0..count)
        .map(|idx| json!({ "ignored": idx, "payload": large }))
        .collect::<Vec<_>>();
    let payload = json!({
        "id": id,
        "title": format!("title-{id}"),
        "created_at": "2026-01-01T00:00:00Z",
        "updated_at": "2026-01-02T00:00:00Z",
        "metadata": {
            "directory": dir,
            "shared": false,
            "share_url": null
        },
        "agent": "default",
        "messages": messages
    });
    let path = sessions.join(format!("{id}.json"));
    std::fs::write(path, serde_json::to_vec(&payload).unwrap()).unwrap();
}
