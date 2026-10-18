//! Oracle command argument schemas, validation orchestration and reporting.
//!
//! [execute] loads source/payload inputs, delegates validation and persistence to
//! the shared RLM crate, and renders text or JSON according to [OracleArgs].
mod args;
mod input;
mod json;
mod output;
mod status;
mod sync;
mod synthetic;
mod validate;
use anyhow::Result;
pub use args::{OracleArgs, OracleCommand, OracleSyncArgs, OracleValidateArgs};
use sync::execute_sync;
use validate::execute_validate;

/// Execute Oracle validation or pending-record synchronization.
///
/// # Arguments
/// * `args` - Validation inputs/output options or a sync request.
/// # Returns
/// Unit after the command completes and prints its result.
/// # Errors
/// Returns invalid input, file/stream I/O, serialization or persistence errors.
/// # Examples
/// ```no_run
/// # async fn example(args: codetether_oracle_cli::OracleArgs) -> anyhow::Result<()> {
/// codetether_oracle_cli::execute(args).await
/// # }
/// ```
pub async fn execute(args: OracleArgs) -> Result<()> {
    match args.command {
        OracleCommand::Validate(v) => execute_validate(v).await,
        OracleCommand::Sync(s) => execute_sync(s).await,
    }
}
