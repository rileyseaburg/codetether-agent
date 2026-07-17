//! Head fill and rolling-tail retention.

use super::Buffer;

pub(super) fn chunk(buffer: &mut Buffer, chunk: &[u8]) {
    if chunk.is_empty() {
        return;
    }
    let head_len = buffer
        .head_budget
        .saturating_sub(buffer.head.len())
        .min(chunk.len());
    buffer.head.extend_from_slice(&chunk[..head_len]);
    tail(buffer, &chunk[head_len..]);
}

fn tail(buffer: &mut Buffer, chunk: &[u8]) {
    if chunk.is_empty() {
        return;
    }
    if buffer.tail_budget == 0 {
        buffer.omitted = buffer.omitted.saturating_add(chunk.len());
        return;
    }
    if chunk.len() >= buffer.tail_budget {
        let start = chunk.len() - buffer.tail_budget;
        buffer.omitted = buffer
            .omitted
            .saturating_add(buffer.tail.len())
            .saturating_add(start);
        buffer.tail.clear();
        buffer.tail.extend(&chunk[start..]);
        return;
    }
    buffer.tail.extend(chunk);
    let excess = buffer.tail.len().saturating_sub(buffer.tail_budget);
    if excess > 0 {
        buffer.tail.drain(..excess);
        buffer.omitted = buffer.omitted.saturating_add(excess);
    }
}
