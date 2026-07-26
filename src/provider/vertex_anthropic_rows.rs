//! Row data for the Vertex AI Anthropic model catalog.
//!
//! Single responsibility: the literal table of models and their limits.
//! Behavior lives in [`super::vertex_anthropic_catalog`].

/// `(id, display_name, context_window, max_output, input_cost, output_cost)`.
pub type Row = (&'static str, &'static str, usize, usize, f64, f64);

/// Claude models served by Vertex AI, newest first.
///
/// Opus 5 leads: 1M context, 128k output, always-on encrypted reasoning.
#[rustfmt::skip]
pub const ROWS: &[Row] = &[
    ("claude-opus-5", "Claude Opus 5", 1_000_000, 128_000, 15.0, 75.0),
    ("claude-sonnet-4-6", "Claude Sonnet 4.6", 200_000, 128_000, 3.0, 15.0),
    ("claude-sonnet-4-20250514", "Claude Sonnet 4", 200_000, 64_000, 3.0, 15.0),
    ("claude-opus-4-20250514", "Claude Opus 4", 200_000, 32_000, 15.0, 75.0),
    ("claude-3-5-sonnet-v2@20241022", "Claude 3.5 Sonnet v2", 200_000, 8_192, 3.0, 15.0),
    ("claude-3-5-sonnet@20240620", "Claude 3.5 Sonnet", 200_000, 8_192, 3.0, 15.0),
    ("claude-3-haiku@20240307", "Claude 3 Haiku", 200_000, 4_096, 0.25, 1.25),
];
