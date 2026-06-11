use super::event::RunEvent;
use anyhow::Result;
use std::io::{self, Write};

pub(super) fn write_stdout(event: &RunEvent<'_>) -> Result<()> {
    let stdout = io::stdout();
    write_event(stdout.lock(), event)
}

pub(super) fn write_event<W: Write>(mut writer: W, event: &RunEvent<'_>) -> Result<()> {
    serde_json::to_writer(&mut writer, event)?;
    writeln!(writer)?;
    Ok(())
}
