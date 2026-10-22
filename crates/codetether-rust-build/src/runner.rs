use crate::{RustAction, RustArgs, plan, process, validate::version_matches};
use anyhow::{Result, ensure};

/// Validate prerequisites and execute with inherited stdin/stdout/stderr.
///
/// Cancellation kills the immediate child on drop (not ra-jit's persistent
/// per-user daemon). Version probes capture stdout and inherit stderr.
/// # Arguments
/// * `args` - Explicit operation; no tools are installed automatically.
/// # Returns
/// Unit only when the operation succeeds.
/// # Errors
/// Returns errors for missing manifests, incompatible rustc, unavailable tools,
/// failed doctor probes, spawn failures or any nonzero child exit status.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_rust_build::{run, RustArgs, RustAction};
/// run(RustArgs { action: RustAction::Check, ..Default::default() }).await?;
/// # Ok(()) }
/// ```
pub async fn run(args: RustArgs) -> Result<()> {
    let command = plan(&args)?;
    if args.action == RustAction::Doctor {
        return crate::doctor::doctor(&args).await;
    }
    if matches!(
        args.action,
        RustAction::CacheBuild | RustAction::CachePublish
    ) {
        crate::cache::validate_workspace(&args.cwd)?;
        process::execute(&command.program, &command.args, &command.cwd).await?;
        for next in &command.after_success {
            process::execute(&command.program, next, &command.cwd).await?;
        }
        return Ok(());
    }
    if matches!(
        args.action,
        RustAction::Build | RustAction::Run | RustAction::Test | RustAction::Check
    ) {
        ensure!(
            args.cwd.join("Cargo.toml").is_file(),
            "Cargo.toml required in {}",
            args.cwd.display()
        );
    }
    if args.action != RustAction::Check {
        let version = process::capture(
            "rustc",
            &[format!("+{}", args.toolchain), "--version".into()],
            &args.cwd,
        )
        .await?;
        ensure!(
            version_matches(&version),
            "ra-jit requires rustc 1.99.0; toolchain '{}' reports {}. Select --toolchain 1.99.0 if installed",
            args.toolchain,
            version.trim()
        );
    }
    process::execute(&command.program, &command.args, &command.cwd).await
}
