//! Evidence collection for semantic synthesis.

use crate::chunker::RlmChunker;
use crate::config::RlmConfig;
use crate::context_index::{self, EvidenceRecord};

/// Bounded evidence pack with source token count.
pub struct Evidence {
    /// Text given to the synthesis model.
    pub text: String,
    /// Estimated tokens in the original source.
    pub input_tokens: usize,
    /// Typed evidence records selected for the query.
    pub records: Vec<EvidenceRecord>,
}

/// Select high-value chunks before model synthesis.
pub fn collect(content: &str, query: &str, source: &str, config: &RlmConfig) -> Evidence {
    let input_tokens = RlmChunker::estimate_tokens(content);
    let budget = semantic_budget(config, input_tokens);
    let index = context_index::load_or_build(source, content);
    let records = context_index::retrieve(&index, query, budget);
    let text = if records.is_empty() {
        RlmChunker::compress(content, budget, None)
    } else {
        render_records(&records)
    };
    Evidence {
        text,
        input_tokens,
        records,
    }
}

fn semantic_budget(config: &RlmConfig, input_tokens: usize) -> usize {
    let base = input_tokens.min(16_000);
    base.max(config.max_iterations.min(20) * 512).max(4_096)
}

fn render_records(records: &[EvidenceRecord]) -> String {
    records
        .iter()
        .map(render_record)
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_record(r: &EvidenceRecord) -> String {
    format!(
        "source={} span={}-{} kind={:?} score={:.1} reason={} symbols={}\n{}\n---",
        r.source,
        r.span.0,
        r.span.1,
        r.kind,
        r.score,
        r.reason,
        r.symbols.join(","),
        r.text
    )
}
