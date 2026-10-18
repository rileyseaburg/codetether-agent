//! Peer startup command.
use super::*;

/// Run the `codetether spawn` command. Owns its own bus and S3 sink, blocks
/// on ctrl_c with graceful shutdown.
pub async fn run(args: SpawnArgs) -> Result<()> {
    codetether_a2a_http::peer_startup::application::run(&args, startup_hooks()).await
}
