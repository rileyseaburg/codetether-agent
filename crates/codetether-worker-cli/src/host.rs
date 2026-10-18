//! Native A2A worker entry points used by CLI coordination.
use codetether_cli_options::A2aArgs;
use codetether_worker_http::WorkerServerState;
use std::future::Future;
/// Operations for starting the existing worker runtime.
/// # Implementors
/// Applications delegate to their standalone and HTTP-integrated worker paths.
pub trait Host {
    /// Run a worker without the optional HTTP server.
    /// # Errors
    /// Returns native worker startup or execution errors.
    fn run(args: A2aArgs) -> impl Future<Output = anyhow::Result<()>>;
    /// Run a worker with state shared by the HTTP server.
    /// # Errors
    /// Returns native worker startup or execution errors.
    fn run_with_state(
        args: A2aArgs,
        state: WorkerServerState,
    ) -> impl Future<Output = anyhow::Result<()>>;
}
