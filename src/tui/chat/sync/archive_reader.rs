//! Archive file reader: batched reads from the chat events JSONL.

use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use anyhow::Result;

/// Read up to ~1 MB from the archive starting at `offset`.
/// Returns `(payload, next_offset, record_count)`.
pub fn read_chat_archive_batch(
    archive_path: &Path,
    offset: u64,
) -> Result<(Vec<u8>, u64, usize)> {
    let mut file = std::fs::File::open(archive_path)?;
    let file_len = file.metadata()?.len();
    if offset >= file_len {
        return Ok((Vec::new(), offset, 0));
    }

    const MAX_BATCH_BYTES: usize = 1024 * 1024;
    let to_read = ((file_len - offset) as usize).min(MAX_BATCH_BYTES);
    file.seek(SeekFrom::Start(offset))?;

    let mut buf = vec![0u8; to_read];
    let bytes_read = file.read(&mut buf)?;
    buf.truncate(bytes_read);

    let records = buf.windows(1).filter(|w| w[0] == b'\n').count();
    Ok((buf, offset + bytes_read as u64, records))
}
