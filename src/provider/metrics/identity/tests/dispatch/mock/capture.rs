//! Request storage shared by the capture-provider dispatch methods.
use crate::provider::CompletionRequest;
use std::sync::{Arc, Mutex};

type Calls = Vec<(CompletionRequest, Option<String>)>;
pub(in super::super) struct Capture(pub Arc<Mutex<Calls>>);

impl Capture {
    pub(super) fn record(&self, request: CompletionRequest, session: Option<&str>) {
        self.0
            .lock()
            .unwrap()
            .push((request, session.map(str::to_owned)));
    }
}
