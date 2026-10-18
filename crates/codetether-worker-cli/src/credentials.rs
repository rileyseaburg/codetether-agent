//! Server-scoped saved-token fallback for worker commands.
use codetether_cli_options::A2aArgs;
pub(super) fn apply(args: &mut A2aArgs) {
    // Auto-load saved credentials if no token provided
    if args.token.is_none()
        && let Some(creds) = codetether_user_auth::load_saved_credentials()
    {
        let target = args.server.trim_end_matches('/');
        let saved = creds.server.trim_end_matches('/');
        if saved == target {
            tracing::info!(email = %creds.email, server = %saved, "Using saved credentials from `codetether auth login`");
            args.token = Some(creds.access_token);
        } else {
            tracing::warn!(saved_server = %saved, target_server = %target, "Ignoring saved credentials for different server");
        }
    }
}
