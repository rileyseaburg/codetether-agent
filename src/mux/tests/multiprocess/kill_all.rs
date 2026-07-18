//! Multi-process proof for concurrent all-session shutdown.

#[tokio::test]
async fn kill_all_stops_every_registered_server() {
    let root = tempfile::tempdir().unwrap();
    let workspace = root.path().join("workspace");
    tokio::fs::create_dir(&workspace).await.unwrap();
    let mut alpha = super::process::start("kill-all-alpha", &workspace, root.path()).await;
    let mut beta = super::process::start("kill-all-beta", &workspace, root.path()).await;
    let records = vec![alpha.record.clone(), beta.record.clone()];

    crate::mux::command::kill_all::run_records(records)
        .await
        .unwrap();

    for child in [&mut alpha.child, &mut beta.child] {
        let status = tokio::time::timeout(std::time::Duration::from_secs(3), child.wait())
            .await
            .expect("mux server should stop")
            .unwrap();
        assert!(status.success());
    }
}
