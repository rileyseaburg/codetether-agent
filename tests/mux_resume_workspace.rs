//! Integration check: resolve the workspace of an on-disk session snapshot.
//!
//! `mux resume` depends on reading `metadata.directory` out of a session
//! snapshot. These exercise real snapshot layouts so schema drift in
//! `SessionMetadata` is caught here rather than at runtime.
//!
//! Session lookup is rooted at `CODETETHER_DATA_DIR`, which is set once for the
//! whole binary. `set_current_dir` is deliberately avoided: it is process-global
//! and races under the default parallel test harness.

use codetether_agent::session::Session;
use std::path::{Path, PathBuf};
use std::sync::Once;

static DATA_DIR: Once = Once::new();

/// Roots session resolution at a stable per-binary data directory.
fn data_root() -> PathBuf {
    let root = std::env::temp_dir().join("ct-mux-resume-tests");
    DATA_DIR.call_once(|| {
        std::fs::create_dir_all(root.join("sessions")).expect("create data dir");
        // SAFETY: set once before any session lookup in this test binary.
        unsafe { std::env::set_var("CODETETHER_DATA_DIR", &root) };
    });
    root
}

/// Writes `snapshot` into the resolvable sessions directory.
fn write_snapshot(id: &str, snapshot: &serde_json::Value) -> PathBuf {
    let sessions = data_root().join("sessions");
    std::fs::create_dir_all(&sessions).expect("create sessions dir");
    let path = sessions.join(format!("{id}.json"));
    std::fs::write(
        &path,
        serde_json::to_string(snapshot).expect("serialize snapshot"),
    )
    .expect("write snapshot");
    path
}

fn assert_same_dir(left: &Path, right: &Path) {
    assert_eq!(
        left.canonicalize().expect("canonicalize left"),
        right.canonicalize().expect("canonicalize right"),
    );
}

#[tokio::test]
async fn resolves_workspace_from_recorded_metadata() {
    let workspace = data_root().join("project-minimal");
    std::fs::create_dir_all(&workspace).expect("create workspace");
    let id = "aaaaaaaa-1111-2222-3333-444444444444";
    write_snapshot(
        id,
        &serde_json::json!({
            "id": id,
            "title": "resume probe",
            "metadata": { "directory": workspace },
            "messages": [],
            "tool_uses": [],
        }),
    );

    let resolved = Session::recorded_workspace(id)
        .await
        .expect("workspace should resolve");

    assert_same_dir(&resolved, &workspace);
}

/// A snapshot carrying the full real-world metadata block must still resolve.
///
/// Regression: reading the workspace via the shared session header failed with
/// `missing field 'shared'`, so the projection must tolerate both minimal and
/// complete metadata.
#[tokio::test]
async fn resolves_workspace_from_complete_metadata_block() {
    let workspace = data_root().join("project-complete");
    std::fs::create_dir_all(&workspace).expect("create workspace");
    let id = "bbbbbbbb-1111-2222-3333-444444444444";
    write_snapshot(
        id,
        &serde_json::json!({
            "id": id,
            "title": "complete metadata",
            "metadata": {
                "directory": workspace,
                "model": "zai/glm-5.2",
                "auto_apply_edits": true,
                "allow_network": false,
                "shared": false,
                "use_worktree": false,
                "slash_autocomplete": false,
            },
            "messages": [],
            "tool_uses": [],
        }),
    );

    let resolved = Session::recorded_workspace(id)
        .await
        .expect("workspace should resolve");

    assert_same_dir(&resolved, &workspace);
}

#[tokio::test]
async fn rejects_a_malformed_session_id() {
    assert!(
        Session::recorded_workspace("../escape").await.is_err(),
        "path traversal must be rejected"
    );
}

#[tokio::test]
async fn reports_a_missing_session() {
    let _ = data_root();
    assert!(
        Session::recorded_workspace("cccccccc-0000-0000-0000-000000000000")
            .await
            .is_err(),
        "an absent snapshot must not resolve"
    );
}
