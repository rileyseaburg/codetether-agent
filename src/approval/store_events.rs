use super::{ApprovalEvent, ApprovalStore};
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

impl ApprovalStore {
    pub(crate) fn append_event(&self, event: ApprovalEvent) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.log_path())?;
        serde_json::to_writer(&mut file, &event)?;
        writeln!(file)?;
        Ok(())
    }

    pub(crate) fn events(&self) -> Result<Vec<ApprovalEvent>> {
        let path = self.log_path();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let file = std::fs::File::open(path)?;
        BufReader::new(file)
            .lines()
            .filter_map(non_empty_line)
            .map(|line| Ok(serde_json::from_str(&line?)?))
            .collect()
    }
}

fn non_empty_line(line: std::io::Result<String>) -> Option<std::io::Result<String>> {
    match line {
        Ok(value) if value.trim().is_empty() => None,
        other => Some(other),
    }
}
