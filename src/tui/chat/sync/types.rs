//! Chat sync UI event types.

#[derive(Debug)]
pub enum ChatSyncUiEvent {
    Status(String),
    Error(String),
    BatchUploaded {
        bytes: u64,
        records: usize,
        object_key: String,
    },
}
