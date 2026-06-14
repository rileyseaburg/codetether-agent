use super::process_types::{MAX_BYTES, PipeOutput};
use std::io::{self, Read};

pub fn read<R: Read>(mut reader: R, id: String, stream: &'static str) -> io::Result<PipeOutput> {
    let mut bytes = Vec::new();
    let mut buf = [0_u8; 8192];
    loop {
        let count = reader.read(&mut buf)?;
        if count == 0 {
            break;
        }
        crate::tool::progress::emit(&id, stream, String::from_utf8_lossy(&buf[..count]).into());
        if bytes.len() <= MAX_BYTES {
            bytes.extend_from_slice(&buf[..count]);
        }
    }
    let truncated = bytes.len() > MAX_BYTES;
    if truncated {
        bytes.truncate(MAX_BYTES);
    }
    Ok(PipeOutput {
        text: String::from_utf8_lossy(&bytes).into_owned(),
        truncated,
    })
}
