//! Construction of process state from PTY or pipe transports.

use anyhow::Result;

use super::super::{Running, SpawnMetadata};

pub(super) fn new(
    child: tokio::process::Child,
    metadata: SpawnMetadata,
    terminal: Option<crate::tool::command_pty::Attached>,
) -> Result<Running> {
    match terminal {
        #[cfg(unix)]
        Some(crate::tool::command_pty::Attached::Pty(master)) => {
            let (stdin, output) = super::super::readers::terminal(master)?;
            Ok(Running {
                process_tree: crate::tool::process_tree::Guard::attach(&child),
                child,
                stdin: Some(stdin),
                output,
                exit_code: None,
                activity: super::super::activity::Activity::new(),
                metadata,
            })
        }
        #[cfg(not(unix))]
        Some(crate::tool::command_pty::Attached::Pipes) => Ok(Running::new(child, metadata)),
        None => Ok(Running::new(child, metadata)),
    }
}
