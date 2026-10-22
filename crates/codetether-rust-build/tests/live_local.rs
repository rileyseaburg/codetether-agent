//! Opt-in real-tool smoke. No user workspace or per-user daemon is modified.
use codetether_rust_build::{RustAction, RustArgs, run};

#[tokio::test]
#[ignore = "requires installed Rust 1.99.0, rust-src and ra-jit; probes prerequisites only"]
async fn installed_toolchain_doctor() {
    run(RustArgs {
        action: RustAction::Doctor,
        cwd: env!("CARGO_MANIFEST_DIR").into(),
        ..Default::default()
    })
    .await
    .unwrap();
}

#[tokio::test]
#[ignore = "real Cargo check of a temporary fixture; opt-in compiler correctness smoke"]
async fn ordinary_check_rejects_borrow_error() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("Cargo.toml"),
        "[workspace]\n[package]\nname='codetether-rust-smoke'\nversion='0.1.0'\nedition='2024'\n[lib]\npath='lib.rs'\n").unwrap();
    std::fs::write(
        directory.path().join("lib.rs"),
        "pub fn bad() { let s = String::from(\"x\"); drop(s); println!(\"{}\", s); }\n",
    )
    .unwrap();
    let args = RustArgs {
        action: RustAction::Check,
        cwd: directory.path().into(),
        ..Default::default()
    };
    let error = run(args.clone()).await.unwrap_err();
    assert!(error.to_string().contains("101"), "{error:#}");
    std::fs::write(
        directory.path().join("lib.rs"),
        "pub fn good() -> u8 { 42 }\n",
    )
    .unwrap();
    run(args).await.unwrap();
}
