//! Decode a closed input contract without accepting command grants.
use anyhow::{Context, Result, ensure};
use codetether_rust_build::RustArgs;
use serde_json::Value;

pub(super) fn parse(input: Value) -> Result<RustArgs> {
    let fields = input.as_object().context("expected an object")?;
    ensure!(
        fields.contains_key("action") && fields.contains_key("cwd"),
        "action and cwd are required"
    );
    let mut merged = serde_json::to_value(RustArgs::default())?;
    for (key, value) in fields {
        if super::metadata::is_runtime_field(key) {
            continue;
        }
        ensure!(
            matches!(
                key.as_str(),
                "action" | "cwd" | "toolchain" | "package" | "bin" | "args"
            ),
            "unsupported field: {key}"
        );
        merged[key] = value.clone();
    }
    let mut args: RustArgs = serde_json::from_value(merged)?;
    ensure!(args.cwd.is_absolute(), "cwd must be absolute");
    args.cwd = args
        .cwd
        .canonicalize()
        .context("workspace directory does not exist")?;
    ensure!(args.cwd.is_dir(), "cwd must be a directory");
    codetether_rust_build::plan(&args)?;
    if matches!(
        serde_json::to_value(args.action)?.as_str(),
        Some("build" | "run" | "test" | "check")
    ) {
        ensure!(
            args.cwd.join("Cargo.toml").is_file(),
            "cwd must contain Cargo.toml"
        );
    }
    Ok(args)
}
