//! Size-bounded JSON frame encoding.

use anyhow::{Context, Result, bail};
use serde::{Serialize, de::DeserializeOwned};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

const MAX_FRAME_BYTES: usize = 1024 * 1024;

pub(in crate::mux) async fn read_frame<T, R>(reader: &mut R) -> Result<Option<T>>
where
    T: DeserializeOwned,
    R: AsyncRead + Unpin,
{
    let mut bytes = Vec::with_capacity(256);
    let mut byte = [0_u8; 1];
    loop {
        let read = reader.read(&mut byte).await.context("read mux frame")?;
        if read == 0 && bytes.is_empty() {
            return Ok(None);
        }
        if read == 0 || byte[0] == b'\n' {
            break;
        }
        if bytes.len() == MAX_FRAME_BYTES {
            bail!("mux frame exceeds {MAX_FRAME_BYTES} bytes");
        }
        bytes.push(byte[0]);
    }
    serde_json::from_slice(&bytes)
        .context("decode mux frame")
        .map(Some)
}

pub(in crate::mux) async fn write_frame<T, W>(writer: &mut W, value: &T) -> Result<()>
where
    T: Serialize,
    W: AsyncWrite + Unpin,
{
    let mut bytes = serde_json::to_vec(value).context("encode mux frame")?;
    if bytes.len() > MAX_FRAME_BYTES {
        bail!("mux frame exceeds {MAX_FRAME_BYTES} bytes");
    }
    bytes.push(b'\n');
    writer.write_all(&bytes).await.context("write mux frame")?;
    writer.flush().await.context("flush mux frame")
}
