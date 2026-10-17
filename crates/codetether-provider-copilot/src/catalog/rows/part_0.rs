//! Ordered catalog rows.
pub(super) const ROWS: &[super::Row] = &[
    ("gpt-4o", "GPT-4o", 128_000, 16_384, true),
    ("gpt-4.1", "GPT-4.1", 128_000, 32_768, false),
    ("gpt-5", "GPT-5", 400_000, 128_000, false),
    ("gpt-5-mini", "GPT-5 mini", 264_000, 64_000, false),
];
