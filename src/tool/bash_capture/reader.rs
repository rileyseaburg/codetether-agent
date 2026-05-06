//! Bounded async stream reader for bash process output.

use super::types::CapturedStream;
use tokio::io::{AsyncRead, AsyncReadExt};

const BUFFER_BYTES: usize = 8192;

pub(super) async fn read_limited<R>(
    mut stream: R,
    max_bytes: usize,
) -> std::io::Result<CapturedStream>
where
    R: AsyncRead + Unpin,
{
    let mut buf = [0_u8; BUFFER_BYTES];
    let mut kept = Vec::with_capacity(max_bytes.min(BUFFER_BYTES));
    let mut bytes = 0;

    loop {
        let read = stream.read(&mut buf).await?;
        if read == 0 {
            break;
        }
        bytes += read;
        append_kept(&mut kept, &buf[..read], max_bytes);
    }

    Ok(CapturedStream {
        text: String::from_utf8_lossy(&kept).into_owned(),
        bytes,
        truncated: bytes > kept.len(),
    })
}

fn append_kept(kept: &mut Vec<u8>, bytes: &[u8], max_bytes: usize) {
    if kept.len() >= max_bytes {
        return;
    }
    let remaining = max_bytes - kept.len();
    kept.extend_from_slice(&bytes[..bytes.len().min(remaining)]);
}
