//! Existing bounded executor regression.
use super::*;

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
