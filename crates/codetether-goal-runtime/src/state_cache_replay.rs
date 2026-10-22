//! Replay a stable file prefix; retain the valid cache on malformed appends.
use super::entry::{Entry, Identity};
use anyhow::{Context, Result, ensure};
use codetether_goal_core::{TaskEvent, TaskState};
use std::{
    fs::{File, Metadata},
    io::{Read, Seek, SeekFrom},
};

pub(super) fn read(file: &mut File, metadata: &Metadata, cached: Option<&Entry>) -> Result<Entry> {
    let identity = Identity::from_metadata(metadata);
    let length = metadata.len();
    let modified = metadata.modified().ok();
    let retained = cached.filter(|entry| {
        identity.reliable() && entry.identity == identity && length >= entry.length
    });
    if let Some(entry) = retained
        .filter(|entry| entry.length == length && modified.is_some() && entry.modified == modified)
    {
        let entry = entry.clone();
        #[cfg(test)]
        let entry = Entry {
            replayed_bytes: 0,
            ..entry
        };
        return Ok(entry);
    }
    let (offset, mut state) = retained
        .filter(|entry| length > entry.length)
        .map_or((0, TaskState::default()), |entry| {
            (entry.length, entry.state.clone())
        });
    file.seek(SeekFrom::Start(offset))?;
    let mut bytes = Vec::new();
    (&mut *file).take(length - offset).read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() as u64 == length - offset,
        "goal journal changed during replay"
    );
    ensure!(
        bytes.is_empty() || bytes.last() == Some(&b'\n'),
        "goal journal has an incomplete event at byte {offset}"
    );
    let text = std::str::from_utf8(&bytes).context("goal journal is not UTF-8")?;
    let mut position = offset;
    for line in text.split_inclusive('\n') {
        if !line.trim().is_empty() {
            let event: TaskEvent = serde_json::from_str(line)
                .with_context(|| format!("invalid goal journal event at byte {position}"))?;
            state.extend(&[event]);
        }
        position += line.len() as u64;
    }
    Ok(Entry {
        _source: std::sync::Arc::new(file.try_clone()?),
        identity,
        length,
        modified,
        state,
        #[cfg(test)]
        replayed_bytes: length - offset,
    })
}
