//! Serialize commands for an explicit second, approval-aware tool invocation.
use anyhow::{Context, Result};
use codetether_rust_build::RustArgs;
use serde_json::{Value, json};

pub(super) fn render(args: RustArgs) -> Result<Value> {
    let action = serde_json::to_value(args.action)?;
    let mut argv = vec![
        "rust".to_owned(),
        "--cwd".into(),
        args.cwd.to_str().context("cwd is not UTF-8")?.into(),
        "--toolchain".into(),
        args.toolchain,
    ];
    for (flag, value) in [("--package", args.package), ("--bin", args.bin)] {
        if let Some(value) = value {
            argv.extend([flag.into(), value]);
        }
    }
    argv.push(action.as_str().context("invalid action")?.into());
    argv.extend(args.args.into_iter().map(|arg| format!("--arg={arg}")));
    let words = std::iter::once("codetether").chain(argv.iter().map(String::as_str));
    let quoted = words
        .map(|word| quote(word, cfg!(windows)))
        .collect::<Vec<_>>()
        .join(" ");
    let (shell, command) = if cfg!(windows) {
        ("powershell", format!("& {quoted}; exit $LASTEXITCODE"))
    } else {
        ("/bin/bash", quoted)
    };
    Ok(json!({
        "executed": false,
        "executable": "codetether", "argv": argv, "cwd": args.cwd,
        "exec_command": {"cmd": command, "workdir": args.cwd, "shell": shell, "yield_time_ms": 1000},
        "next": "Submit exec_command separately with a justification and normal approval; use write_stdin for any returned session_id.",
        "effects": "Build/run/test/check may execute workspace code and write artifacts. cache-build/cache-publish execute the repository release pipeline and may upload verified artifacts using host credentials; publication may be deferred by repository guards. stop affects the shared per-user daemon. No approval is granted by this plan.",
        "validation": "not-run; ra-jit results do not replace normal Cargo checks or measure editor latency"
    }))
}

pub(super) fn quote(word: &str, powershell: bool) -> String {
    let escaped = if powershell {
        word.replace('\'', "''")
    } else {
        word.replace('\'', "'\\''")
    };
    format!("'{escaped}'")
}
