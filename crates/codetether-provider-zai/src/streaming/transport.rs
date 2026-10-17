//! Retry classification for interrupted HTTP streaming transports.
use crate::StreamChunk;

pub(crate) fn error(error: reqwest::Error) -> Vec<StreamChunk> {
    vec![StreamChunk::Error(crate::stream_fault::retryable(error))]
}
