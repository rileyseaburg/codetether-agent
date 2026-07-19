//! Error wrapper carrying output items completed before retry exhaustion.

use crate::provider::ContentPart;

#[derive(Debug, thiserror::Error)]
#[error("{source}")]
struct Checkpointed {
    source: anyhow::Error,
    content: Vec<ContentPart>,
}

pub(super) fn wrap(source: anyhow::Error, content: Vec<ContentPart>) -> anyhow::Error {
    if content.is_empty() {
        return source;
    }
    anyhow::Error::new(Checkpointed { source, content })
}

pub(super) fn content(error: &anyhow::Error) -> Option<Vec<ContentPart>> {
    error
        .downcast_ref::<Checkpointed>()
        .map(|checkpointed| checkpointed.content.clone())
}
