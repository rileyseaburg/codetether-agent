//! Wait for a detached PTY proof file.

pub(super) async fn wait_for(path: &std::path::Path) {
    // Job containers boot on a loaded 2-CPU LXC; allow up to 5s for the proof file.
    for _ in 0..500 {
        if path.exists() {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    panic!("detached PTY did not write the proof file within 5s");
}
