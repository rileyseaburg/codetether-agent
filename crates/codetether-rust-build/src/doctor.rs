use crate::{RustArgs, process::capture, validate::version_matches};
use anyhow::{Result, ensure};
use std::io::{Write, stdout};

pub(crate) async fn doctor(args: &RustArgs) -> Result<()> {
    let tc = format!("+{}", args.toolchain);
    let probes = [
        ("cargo", "cargo", vec![tc.clone(), "--version".into()]),
        (
            "rustc (requires 1.99.0)",
            "rustc",
            vec![tc.clone(), "--version".into()],
        ),
        (
            "ra-jit",
            "cargo",
            vec![tc.clone(), "ra-jit".into(), "--version".into()],
        ),
        (
            "rust-src",
            "rustc",
            vec![tc, "--print".into(), "sysroot".into()],
        ),
    ];
    let mut failures = 0;
    for (label, program, argv) in probes {
        let result = capture(program, &argv, &args.cwd).await.and_then(|text| {
            if label.starts_with("rustc") {
                ensure!(
                    version_matches(&text),
                    "expected rustc 1.99.0, found {}",
                    text.trim()
                );
            }
            if label == "rust-src" {
                let source = std::path::Path::new(text.trim())
                    .join("lib/rustlib/src/rust/library/core/src/lib.rs");
                ensure!(source.is_file(), "rust-src missing under {}", text.trim());
                return Ok(source.display().to_string());
            }
            Ok(text.trim().to_owned())
        });
        match result {
            Ok(detail) => writeln!(stdout().lock(), "OK {label}: {detail}")?,
            Err(error) => {
                failures += 1;
                writeln!(stdout().lock(), "MISSING/INCOMPATIBLE {label}: {error:#}")?;
            }
        }
    }
    ensure!(
        failures == 0,
        "doctor found {failures} missing or incompatible prerequisites; no installation or configuration changes made"
    );
    Ok(())
}
