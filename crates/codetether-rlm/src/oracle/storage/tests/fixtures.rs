//! Mock remote backends and sample record builders for tests.
//!
//! Provides `MockOk` (always succeeds), `MockErr` (always fails),
//! and [`sample`] for constructing test `OracleTraceRecord`s.
//!
//! # Examples
//!
//! ```ignore
//! let record = sample("golden");
//! let remote: Arc<dyn OracleRemote> = Arc::new(MockOk);
//! ```

use super::super::remote::OracleRemote;
use crate::oracle::record::OracleTraceRecord;
use crate::oracle::{FinalPayload, SemanticPayload, ValidatedTrace, VerificationMethod};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;

pub const MAX: u64 = 500 * 1024 * 1024;

pub struct MockOk;

#[async_trait]
impl OracleRemote for MockOk {
    async fn upload_record(&self, _: &OracleTraceRecord) -> Result<(String, String)> {
        Ok(("oracle/key.jsonl".into(), "http://example/key.jsonl".into()))
    }
}

pub struct MockErr;

#[async_trait]
impl OracleRemote for MockErr {
    async fn upload_record(&self, _: &OracleTraceRecord) -> Result<(String, String)> {
        anyhow::bail!("simulated failure")
    }
}

pub fn sample(verdict: &str) -> OracleTraceRecord {
    OracleTraceRecord {
        verdict: verdict.into(),
        reason: Some("reason".into()),
        agreement_ratio: None,
        trace: ValidatedTrace {
            prompt: "Find async fns".into(),
            trace: vec![],
            final_payload: Some(FinalPayload::Semantic(SemanticPayload {
                file: "src/main.rs".into(),
                answer: "answer".into(),
            })),
            verdict: verdict.into(),
            oracle_diff: None,
            repo_revision: "abc123".into(),
            timestamp: Utc::now().to_rfc3339(),
            answer: "answer".into(),
            iterations: 1,
            subcalls: 0,
            input_tokens: 1,
            output_tokens: 1,
            elapsed_ms: 1,
            source_path: Some("src/main.rs".into()),
            verification_method: VerificationMethod::None,
            trace_id: uuid::Uuid::new_v4().to_string(),
        },
    }
}
