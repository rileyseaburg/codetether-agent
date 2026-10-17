//! Process-tree cancellation integration coverage.

use std::path::Path;

use tokio::time::{Duration, Instant, sleep};

use super::{Guard, configure};
#[cfg(windows)]
#[path = "windows_tests_fixture.rs"]
mod fixture;
#[cfg(unix)]
#[path = "unix_tests_fixture.rs"]
mod fixture;
#[cfg(any(unix, windows))]
use fixture::descendant_command;

/// Proves dropping an armed guard prevents a descendant from surviving.
#[cfg(any(unix, windows))]
#[tokio::test]
async fn dropping_guard_kills_descendant_tree() {
    let temp = tempfile::tempdir().expect("create process-tree fixture");
    let started = temp.path().join("started.txt");
    let survived = temp.path().join("survived.txt");
    let mut command = descendant_command(&temp, &started, &survived);
    configure(&mut command);
    let child = command.spawn().expect("spawn process-tree fixture");
    let guard = Guard::attach(&child);

    wait_for_file(&started).await;
    drop(guard);
    drop(child);
    sleep(Duration::from_millis(1_600)).await;

    assert!(
        !survived.exists(),
        "descendant continued after its process-tree guard was dropped"
    );
}

/// Waits for a descendant marker without relying on fixed startup timing.
async fn wait_for_file(path: &Path) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !path.exists() && Instant::now() < deadline {
        sleep(Duration::from_millis(20)).await;
    }
    assert!(path.exists(), "descendant process never started");
}
