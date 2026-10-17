//! Model and mode identifiers used by Gemini Web request routing.

/// (model_id, mode_id, display_name, context_window)
pub const MODELS: &[(&str, &str, &str, usize)] = &[
    (
        "gemini-web-fast",
        "fbb127bbb056c959",
        "Gemini 3 Fast",
        1_048_576_usize,
    ),
    (
        "gemini-web-thinking",
        "5bf011840784117a",
        "Gemini 3 Thinking",
        1_048_576_usize,
    ),
    (
        "gemini-web-pro",
        "9d8ca3786ebdfbea",
        "Gemini 3.1 Pro",
        1_048_576_usize,
    ),
    (
        "gemini-web-deep-think",
        "e6fa609c3fa255c0",
        "Gemini 3 Deep Think",
        1_048_576_usize,
    ),
];
