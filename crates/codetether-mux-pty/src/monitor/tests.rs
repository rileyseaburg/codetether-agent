//! Linux child-reaping regression tests.

use std::process::Command;
use std::time::{Duration, Instant};

#[test]
fn shared_reaper_collects_exited_child() {
    let child = Command::new("/bin/sh")
        .arg("-c")
        .arg("exit 0")
        .spawn()
        .unwrap();
    let process_path = format!("/proc/{}", child.id());
    super::start(child);
    let deadline = Instant::now() + Duration::from_secs(1);
    while std::path::Path::new(&process_path).exists() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(!std::path::Path::new(&process_path).exists());
}
