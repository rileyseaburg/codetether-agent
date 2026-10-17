//! Stdin delivery to a session that still owns an open input handle.

use anyhow::{Context, Result};
use tokio::io::AsyncWriteExt;

use super::Running;

pub(super) async fn write(command: &mut Running, chars: &str) -> Result<()> {
    let stdin = command.stdin.as_mut().context(
        "stdin is closed for this session; rerun exec_command with tty=true to keep stdin open",
    )?;
    stdin.as_mut().write_all(chars.as_bytes()).await?;
    stdin.as_mut().flush().await?;
    Ok(())
}
