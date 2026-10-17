//! Ordered catalog rows.
pub(super) const ROWS: &[super::Row] = &[
    ("gpt-5.1-codex-mini", ("GPT-5.1-Codex-Mini", 0.33)),
    ("gpt-5.1-codex-max", ("GPT-5.1-Codex-Max", 1.0)),
    ("gpt-5", ("GPT-5", 1.0)),
    ("gpt-5-mini", ("GPT-5 mini", 0.0)),
    ("gpt-5-codex", ("GPT-5-Codex", 1.0)),
    ("gpt-4.1", ("GPT-4.1", 0.0)),
    ("gpt-4o", ("GPT-4o", 0.0)),
    ("gemini-2.5-pro", ("Gemini 2.5 Pro", 1.0)),
    ("gemini-3.1-pro-preview", ("Gemini 3.1 Pro Preview", 1.0)),
    (
        "gemini-3.1-pro-preview-customtools",
        ("Gemini 3.1 Pro Preview (Custom Tools)", 1.0),
    ),
    ("gemini-3-flash-preview", ("Gemini 3 Flash Preview", 0.33)),
    ("gemini-3-pro-preview", ("Gemini 3 Pro Preview", 1.0)),
];
