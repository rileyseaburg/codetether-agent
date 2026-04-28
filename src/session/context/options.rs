//! Options for derived-context provider requests.

/// Tunables for a derived-context provider request.
#[derive(Clone, Copy, Default)]
pub struct RequestOptions {
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub max_tokens: Option<usize>,
    pub force_keep_last: Option<usize>,
}
