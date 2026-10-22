//! Mocked-local pipeline tests: no remote cache or real release build.
#![cfg(unix)]
use codetether_rust_build::{RustAction, RustArgs, plan, run};
use std::fs;

fn fixture(script: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("Cargo.toml"), "[workspace]\n").unwrap();
    fs::create_dir_all(dir.path().join("scripts/dev")).unwrap();
    fs::write(dir.path().join("scripts/dev/release-build-loop.sh"), script).unwrap();
    dir
}

fn args(dir: &tempfile::TempDir, action: RustAction) -> RustArgs {
    RustArgs {
        cwd: dir.path().into(),
        action,
        ..Default::default()
    }
}

#[tokio::test]
async fn build_then_publish_and_publish_only() {
    let dir = fixture("printf '%s\\n' \"$1\" >> calls\n");
    run(args(&dir, RustAction::CacheBuild)).await.unwrap();
    run(args(&dir, RustAction::CachePublish)).await.unwrap();
    assert_eq!(
        fs::read_to_string(dir.path().join("calls")).unwrap(),
        "build\npublish\npublish\n"
    );
}

#[tokio::test]
async fn failed_build_never_publishes_and_publication_failure_propagates() {
    let dir = fixture("printf '%s\\n' \"$1\" >> calls\nexit 7\n");
    assert!(run(args(&dir, RustAction::CacheBuild)).await.is_err());
    assert_eq!(
        fs::read_to_string(dir.path().join("calls")).unwrap(),
        "build\n"
    );
    assert!(run(args(&dir, RustAction::CachePublish)).await.is_err());
}

#[test]
fn cache_rejects_selectors_and_custom_toolchains() {
    let dir = fixture("");
    for action in [RustAction::CacheBuild, RustAction::CachePublish] {
        let base = args(&dir, action);
        for input in [
            RustArgs {
                package: Some("app".into()),
                ..base.clone()
            },
            RustArgs {
                bin: Some("app".into()),
                ..base.clone()
            },
            RustArgs {
                args: vec!["--force".into()],
                ..base.clone()
            },
            RustArgs {
                toolchain: "nightly".into(),
                ..base
            },
        ] {
            assert!(plan(&input).is_err());
        }
    }
}
