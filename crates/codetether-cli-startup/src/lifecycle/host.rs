//! Native parser, early-command, crash, and final-dispatch operations.
use anyhow::Result;
use codetether_cli_command::Command;
use std::future::Future;

/// Application operations required by the shared CLI startup lifecycle.
///
/// # Implementors
/// Native binaries retain their root argument parser, version metadata,
/// process-specific crash setup, and concrete command dispatch.
pub trait Host {
    /// Parsed root options and selected command.
    type Invocation;
    /// Run commands that bypass environment setup and full startup.
    /// By default, only `windows` and `bonsai` in the first argument position
    /// are parsed and dispatched before the normal startup sequence.
    fn early() -> impl Future<Output = Option<Result<()>>>
    where
        Self: Sized,
    {
        super::early::run::<Self>()
    }
    /// Parse the application's complete root command line.
    fn parse() -> Self::Invocation;
    /// Borrow the selected command for logging and optional-service policy.
    fn command(invocation: &Self::Invocation) -> &Option<Command>;
    /// Handle native commands before initializing logging and services.
    fn native(invocation: &Self::Invocation) -> impl Future<Output = Option<Result<()>>>;
    /// Initialize crash reporting with the application's version and guards.
    fn initialize_crash(is_tui: bool) -> impl Future<Output = ()>;
    /// Execute the parsed invocation after startup completes.
    fn dispatch(invocation: Self::Invocation) -> impl Future<Output = Result<()>>;
}
