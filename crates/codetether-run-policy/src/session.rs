//! CLI edit and step policy applied to the session.
use anyhow::Result;
use codetether_cli_options::RunArgs;

/// Applies CLI-driven session policy from `args` onto `session`.
///
/// * `--yolo` enables auto-applying edits without prompting.
/// * `--max-steps` is validated (must be at least 1) and copied onto the
///   session.
///
/// # Errors
///
/// Returns an error if `--max-steps` is zero.
/// Apply automatic-edit and step-budget policy to a session.
/// # Arguments
/// * `session` - Session receiving policy changes.
/// * `args` - CLI edit and step-budget options.
/// # Returns
/// Unit after policy application.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_run_policy::SessionPolicy>(session: &mut S, args: &codetether_cli_options::RunArgs) -> anyhow::Result<()> {
/// codetether_run_policy::apply_session_policy(session, args)?;
/// # Ok(())
/// # }
/// ```
pub fn apply_session_policy<S: crate::SessionPolicy>(
    session: &mut S,
    args: &RunArgs,
) -> Result<()> {
    if args.yolo {
        session.enable_auto_apply();
    }
    if let Some(0) = args.max_steps {
        anyhow::bail!("--max-steps must be at least 1");
    }
    session.set_max_steps(args.max_steps);
    Ok(())
}
