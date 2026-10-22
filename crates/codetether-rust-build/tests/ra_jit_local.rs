//! Opt-in real ra-jit smoke with a private daemon and no user-workspace changes.
use codetether_rust_build::{RustAction, RustArgs, run};

#[tokio::test]
#[ignore = "requires installed ra-jit and Rust 1.99.0; creates a private fixture daemon"]
async fn isolated_build_run() {
    let directory = tempfile::tempdir().unwrap();
    let project = directory.path().join("project");
    std::fs::create_dir(&project).unwrap();
    std::fs::write(project.join("Cargo.toml"),
        "[workspace]\n[package]\nname='codetether-ra-smoke'\nversion='0.1.0'\nedition='2024'\n[[bin]]\nname='smoke'\npath='main.rs'\n").unwrap();
    std::fs::write(project.join("main.rs"),
        "fn main() { assert_eq!(std::env::args().nth(1).as_deref(), Some(\"literal a;b\")); println!(\"fixture-ok\"); }\n").unwrap();
    let home = directory.path().join("daemon");
    let status = tokio::process::Command::new(std::env::current_exe().unwrap())
        .args(["--ignored", "--exact", "fixture_child", "--nocapture"])
        .env("CODETETHER_RA_FIXTURE", &project)
        .env("RA_JIT_HOME", &home)
        .env("RA_JIT_CACHE", "off")
        .env("RA_JIT_IDLE_SECS", "10")
        .kill_on_drop(true)
        .status()
        .await
        .unwrap();
    let cleanup = tokio::process::Command::new("cargo")
        .args(["+stable", "ra-jit", "stop"])
        .env("RA_JIT_HOME", &home)
        .env("RA_JIT_CACHE", "off")
        .env("RUSTUP_AUTO_INSTALL", "0")
        .current_dir(&project)
        .kill_on_drop(true)
        .status()
        .await
        .unwrap();
    assert!(cleanup.success(), "private daemon cleanup failed");
    assert!(status.success(), "ra-jit fixture subprocess failed");
}

#[tokio::test]
#[ignore = "child fixture; run through isolated_build_run for environment isolation"]
async fn fixture_child() {
    let Ok(project) = std::env::var("CODETETHER_RA_FIXTURE") else {
        return;
    };
    let input = RustArgs {
        cwd: project.into(),
        bin: Some("smoke".into()),
        ..Default::default()
    };
    run(input.clone()).await.unwrap();
    run(input.clone()).await.unwrap();
    run(RustArgs {
        action: RustAction::Run,
        args: vec!["literal a;b".into()],
        ..input.clone()
    })
    .await
    .unwrap();
    run(RustArgs {
        action: RustAction::Status,
        cwd: input.cwd,
        ..Default::default()
    })
    .await
    .unwrap();
}
