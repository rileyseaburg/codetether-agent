//! Multi-process proof: killing sessions one at a time stops the server last.

#[tokio::test]
async fn kill_closes_one_session_and_the_last_kill_stops_the_server() {
    let _env = crate::approval::test_env::lock_env();
    let root = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", root.path()) };
    let workspace = tokio::fs::canonicalize(root.path()).await.unwrap();
    let mut first = super::process::start("kill-one", &workspace, root.path()).await;
    let record = crate::mux::control::start_join::live_server(&workspace)
        .await
        .unwrap();
    crate::mux::control::start_join::create_session(record, "kill-two", workspace.clone())
        .await
        .unwrap();

    crate::mux::control::stop_session("kill-two").await.unwrap();
    let after = super::registry::find_session(root.path(), "kill-one")
        .await
        .expect("server still hosts the remaining session");
    assert_eq!(after.record.pid, first.target.record.pid);
    assert!(after.record.state.session("kill-two").is_none());
    assert!(
        first.child.try_wait().unwrap().is_none(),
        "server must stay up"
    );

    crate::mux::control::stop_session("kill-one").await.unwrap();
    let status = tokio::time::timeout(std::time::Duration::from_secs(3), first.child.wait())
        .await
        .expect("server stops after its last session closes")
        .unwrap();
    assert!(status.success());
    assert!(
        super::registry::find_session(root.path(), "kill-one")
            .await
            .is_none()
    );
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
}
