//! Construction of process state from PTY or pipe transports.

use anyhow::Result;

use super::super::{Running, SpawnMetadata};

pub(super) fn new(
    child: tokio::process::Child,
    metadata: SpawnMetadata,
    terminal: Option<codetether_command_pty::Attached>,
) -> Result<Running> {
    match terminal {
        #[cfg(unix)]
        Some(codetether_command_pty::Attached::Pty(master)) => {
            let (stdin, output) = super::super::readers::terminal(master)?;
            Ok(Running {
                process_tree: codetether_process_tree::Guard::attach(&child),
                child,
                stdin: Some(stdin),
                output,
                exit_code: None,
                activity: super::super::activity::Activity::new(),
                metadata,
            })
        }
        #[cfg(not(unix))]
        Some(codetether_command_pty::Attached::Pipes) => Ok(Running::new(child, metadata)),
        None => Ok(Running::new(child, metadata)),
    }
}
