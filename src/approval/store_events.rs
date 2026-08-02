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
        Ok(())
    }

    /// Read every event, skipping lines that cannot be parsed.
    ///
    /// A single corrupt line must not make the whole store unreadable. Logs
    /// predating the atomic-append fix contain interleaved records, and failing
    /// the entire read leaves an operator with no way to see or decide any
    /// pending request. Unparseable lines are reported on stderr and skipped, so
    /// the damage is visible without being fatal.
    ///
    /// # Errors
    ///
    /// Returns an error only when the log cannot be read at all.
    pub(crate) fn events(&self) -> Result<Vec<ApprovalEvent>> {
        let path = self.log_path();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let file = std::fs::File::open(path)?;
        let mut events = Vec::new();
        let mut skipped = 0usize;
        for line in BufReader::new(file).lines().filter_map(non_empty_line) {
            match serde_json::from_str(&line?) {
                Ok(event) => events.push(event),
                Err(_) => skipped += 1,
            }
        }
        if skipped > 0 {
            eprintln!(
                "warning: skipped {skipped} unreadable approval log line(s) in {}",
                self.log_path().display()
            );
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
