//! Types exchanged by the asynchronous history-page loader.

use crate::provider::Message;

pub(super) type Fingerprint = u64;
pub(super) type PageResult = (u64, Result<Page, String>);

pub(super) struct Page {
    pub(super) messages: Vec<Message>,
    pub(super) boundary: Vec<Fingerprint>,
    pub(super) exhausted: bool,
    pub(super) depth: usize,
}

pub(super) struct Request {
    pub generation: u64,
    pub source_id: String,
    pub boundary: Vec<Fingerprint>,
    pub depth: usize,
}
