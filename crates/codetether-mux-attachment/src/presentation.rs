//! Writes ordered PTY output to the local terminal.

use std::io::Write;

use anyhow::Result;

pub(super) fn write(data: &[u8]) -> Result<()> {
    let mut stdout = std::io::stdout();
    stdout.write_all(data)?;
    stdout.flush()?;
    Ok(())
}
