//! Pure validation behavior captured before and after module consolidation.
use crate::oracle::{OracleResult, TraceValidator};
use crate::{RlmAnalysisResult, SubQuery};
mod batch;
mod patterns;
mod routing;
fn analysis(query: &str, answer: &str) -> RlmAnalysisResult {
    RlmAnalysisResult {
        answer: answer.into(),
        iterations: 2,
        sub_queries: vec![SubQuery {
            query: query.into(),
            context_slice: None,
            response: String::new(),
            tokens_used: 3,
        }],
        stats: Default::default(),
    }
}
fn capture(result: OracleResult) -> serde_json::Value {
    use serde_json::json;
    let (kind, reason, diff, ratio, mut trace) = match result {
        OracleResult::Golden(t) => ("golden", None, None, None, t),
        OracleResult::Unverified { reason, trace } => {
            ("unverified", Some(reason), None, None, trace)
        }
        OracleResult::Failed {
            reason,
            diff,
            trace,
        } => ("failed", Some(reason), diff, None, trace),
        OracleResult::Consensus {
            trace,
            agreement_ratio,
        } => ("consensus", None, None, Some(agreement_ratio), trace),
    };
    trace.timestamp = "fixed".into();
    trace.trace_id = "fixed".into();
    json!({"kind":kind,"reason":reason,"diff":diff,"ratio":ratio,"trace":trace})
}

mod cases;

mod comparisons;
