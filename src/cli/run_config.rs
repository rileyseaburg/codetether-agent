use super::RunArgs;
use crate::config::{AccessMode, Config};
use crate::session::Session;
use anyhow::Result;

pub(super) async fn load(args: &RunArgs) -> Config {
    let mut config = Config::load().await.unwrap_or_default();
    let access_mode = effective_access_mode(args);
    Config::apply_process_access_mode_override(access_mode);
    config.apply_access_mode_override(access_mode);
    config
}

/// Return the effective access mode, letting `--yolo` act as `--access-mode full`.
pub(super) fn effective_access_mode(args: &RunArgs) -> Option<AccessMode> {
    if args.yolo {
        Some(AccessMode::Full)
    } else {
        args.access_mode
    }
}

/// Applies CLI-driven session policy from `args` onto `session`.
///
/// * `--yolo` enables auto-applying edits without prompting.
/// * `--max-steps` is validated (must be at least 1) and copied onto the
///   session.
///
/// # Errors
///
/// Returns an error if `--max-steps` is zero.
pub(super) fn apply_session_policy(session: &mut Session, args: &RunArgs) -> Result<()> {
    if args.yolo {
        session.metadata.auto_apply_edits = true;
    }
    if let Some(0) = args.max_steps {
        anyhow::bail!("--max-steps must be at least 1");
    }
    session.max_steps = args.max_steps;
    Ok(())
}
