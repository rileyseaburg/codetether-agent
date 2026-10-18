//! Application binding for background forage execution.
use super::{host::Runtime, state::ForageUpdate};
use crate::cli::ForageArgs;
/// Launch forage using the application engine.
pub fn spawn_forage_run(args: ForageArgs, tx: tokio::sync::mpsc::Sender<ForageUpdate>) {
    codetether_forage_interactive::spawn_forage_run::<Runtime>(args, tx);
}
