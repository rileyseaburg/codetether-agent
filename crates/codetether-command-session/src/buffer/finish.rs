//! Model-facing rendering of retained head and tail bytes.

use super::Buffer;

pub(super) fn output(buffer: Buffer) -> (String, usize) {
    let mut bytes = Vec::with_capacity(buffer.head.len() + buffer.tail.len() + 32);
    bytes.extend_from_slice(&buffer.head);
    if buffer.omitted > 0 {
        bytes.extend_from_slice(format!("\n... {} bytes omitted ...\n", buffer.omitted).as_bytes());
    }
    bytes.extend(buffer.tail);
    (String::from_utf8_lossy(&bytes).into_owned(), buffer.omitted)
}
