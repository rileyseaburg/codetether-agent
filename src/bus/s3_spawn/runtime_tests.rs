use super::{initialize, parse};

#[test]
fn thread_count_defaults_and_clamps() {
    assert_eq!(parse(None), 2);
    assert_eq!(parse(Some("0")), 1);
    assert_eq!(parse(Some("99")), 8);
    assert_eq!(parse(Some("invalid")), 2);
}

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

#[test]
#[cfg(target_os = "linux")]
fn async_std_executor_child() {
    if std::env::var("CODETETHER_ASYNC_STD_EXECUTOR_CHILD").as_deref() != Ok("1") {
        return;
    }
    initialize();
    async_std::task::block_on(async { async_std::task::spawn(async {}).await });

    let names = thread_names();
    assert!(!names.iter().any(|name| name.starts_with("async-std/runti")));
    let bounded = names.iter().filter(|name| *name == "codetether-s3").count();
    assert!((1..=8).contains(&bounded), "thread names: {names:?}");
}

#[cfg(target_os = "linux")]
fn thread_names() -> Vec<String> {
    std::fs::read_dir("/proc/self/task")
        .expect("read process tasks")
        .filter_map(Result::ok)
        .filter_map(|entry| std::fs::read_to_string(entry.path().join("comm")).ok())
        .map(|name| name.trim().to_string())
        .collect()
}
