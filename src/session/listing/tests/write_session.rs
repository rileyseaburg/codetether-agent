//! Shared fixture: write a session JSON with a large `messages` payload.

use serde_json::json;

pub(super) fn write_session(
    sessions: &std::path::Path,
    id: &str,
    dir: &std::path::Path,
    count: usize,
) {
    let large = "x".repeat(1024 * 1024);
    let messages = (0..count)
        .map(|idx| json!({ "ignored": idx, "payload": large }))
        .collect::<Vec<_>>();
    let payload = json!({
        "id": id,
        "title": format!("title-{id}"),
        "created_at": "2026-01-01T00:00:00Z",
        "updated_at": "2026-01-02T00:00:00Z",
        "metadata": { "directory": dir, "shared": false, "share_url": null },
        "agent": "default",
        "messages": messages
    });
    let path = sessions.join(format!("{id}.json"));
    std::fs::write(path, serde_json::to_vec(&payload).unwrap()).unwrap();
}
