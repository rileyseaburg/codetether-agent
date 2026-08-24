use super::{ApprovalEvent, ApprovalStore};
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

impl ApprovalStore {
    /// Append one event as a single line.
    ///
    /// The record is serialized into a buffer and written with one `write_all`,
    /// rather than streamed through `serde_json::to_writer`. Streaming issues many
    /// small writes, and two processes appending at once interleave them, which
    /// corrupts the log into unparseable text like `{{""eventevent""` and makes
    /// every later read fail. One write to a file opened in append mode keeps a
    /// record intact.
    ///
    /// # Errors
    ///
    /// Returns an error when the log cannot be opened or written.
    pub(crate) fn append_event(&self, event: ApprovalEvent) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.log_path())?;
        let mut line = serde_json::to_vec(&event)?;
        line.push(b'\n');
        file.write_all(&line)?;
        file.sync_all()?;
        Ok(())
    }

    /// Read every event, failing closed when any line cannot be parsed.
    ///
    /// Skipping a corrupt decision could resurrect an earlier approval, so
    /// operators must repair corrupted logs before authorization can resume.
    ///
    /// # Errors
    ///
    /// Returns an error when the log cannot be read or any event is malformed.
    pub(crate) fn events(&self) -> Result<Vec<ApprovalEvent>> {
        let path = self.log_path();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let file = std::fs::File::open(path)?;
        let mut events = Vec::new();
        for line in BufReader::new(file).lines().filter_map(non_empty_line) {
            events.push(serde_json::from_str(&line?)?);
        }
        Ok(events)
    }
}

fn non_empty_line(line: std::io::Result<String>) -> Option<std::io::Result<String>> {
    match line {
        Ok(value) if value.trim().is_empty() => None,
        other => Some(other),
    }
}
