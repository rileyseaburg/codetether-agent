use super::record::WorktreeRecord;
use std::path::PathBuf;

pub(super) fn records(output: &[u8]) -> Vec<WorktreeRecord> {
    let mut records = Vec::new();
    let mut current = None;
    for raw in output.split(|byte| *byte == 0) {
        if raw.is_empty() {
            finish(&mut records, &mut current);
            continue;
        }
        let line = String::from_utf8_lossy(raw);
        let record = current.get_or_insert_with(WorktreeRecord::default);
        if let Some(value) = line.strip_prefix("worktree ") {
            record.path = PathBuf::from(value);
        } else if let Some(value) = line.strip_prefix("HEAD ") {
            record.head = value.into();
        } else if let Some(value) = line.strip_prefix("branch refs/heads/") {
            record.branch = Some(value.into());
        } else if line.starts_with("locked") {
            record.locked = true;
        } else if line.starts_with("prunable") {
            record.prunable = true;
        }
    }
    finish(&mut records, &mut current);
    if let Some(first) = records.first_mut() {
        first.primary = true;
    }
    records
}

fn finish(records: &mut Vec<WorktreeRecord>, current: &mut Option<WorktreeRecord>) {
    if let Some(record) = current.take()
        && !record.path.as_os_str().is_empty()
    {
        records.push(record);
    }
}
