//! Stream choice content and completion markers.
use crate::*;
pub(super) fn push(choice: &StreamChoice, chunks: &mut Vec<StreamChunk>) {
    if let Some(content) = &choice.delta.content {
        chunks.push(StreamChunk::Text(content.clone()));
    }

    super::tools::push(choice, chunks);
    if choice.finish_reason.is_some() {
        chunks.push(StreamChunk::Done { usage: None });
    }
}
