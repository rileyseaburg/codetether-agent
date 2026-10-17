//! Existing bounded executor regression.
use super::*;

#[test]
#[cfg(target_os = "linux")]
fn async_std_uses_the_bounded_s3_executor() {
    let status = std::process::Command::new(std::env::current_exe().expect("current test binary"))
        .args([
            "async_std_executor_child",
            "--nocapture",
            "--test-threads=1",
        ])
        .env("CODETETHER_ASYNC_STD_EXECUTOR_CHILD", "1")
        .status()
        .expect("run isolated executor test");
    assert!(status.success());
}
