//! One backend attempt and its execution evidence.
use crate::{Backend, SearchFailure, WebSearchTool, result::SearchResult};
use serde_json::{Value, json};

/// What one backend request actually returned.
pub(crate) struct Attempt {
    pub(crate) backend: Backend,
    pub(crate) status: Option<u16>,
    pub(crate) request_id: Option<String>,
    pub(crate) outcome: Result<Vec<SearchResult>, SearchFailure>,
}

impl Attempt {
    /// Evidence object recorded for this attempt.
    pub(crate) fn evidence(&self) -> Value {
        let (outcome, count, detail) = match &self.outcome {
            Ok(results) => ("ok", Some(results.len()), None),
            Err(failure) => (failure.code(), None, Some(failure.detail())),
        };
        json!({
            "backend": self.backend.id(),
            "http_status": self.status,
            "request_id": self.request_id,
            "outcome": outcome,
            "result_count": count,
            "detail": detail,
        })
    }
}

impl WebSearchTool {
    /// Contact one backend and classify its response.
    pub(crate) async fn attempt(&self, backend: Backend, query: &str, max: usize) -> Attempt {
        let (url, id_header) = backend.request(query);
        match self.fetch(&url, id_header).await {
            Err(failure) => Attempt {
                backend,
                status: None,
                request_id: None,
                outcome: Err(failure),
            },
            Ok(page) => Attempt {
                backend,
                status: Some(page.status),
                outcome: backend.parse(page.status, &page.body, max),
                request_id: page.request_id,
            },
        }
    }
}
