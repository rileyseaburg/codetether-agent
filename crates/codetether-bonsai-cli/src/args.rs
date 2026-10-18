//! Arguments for direct native generation, separate from application startup.
use clap::Args;
/// Direct Bonsai request options.
/// # Examples
/// ```text
/// codetether bonsai --prompt "Reply with BONSAI_NATIVE_OK" --repeat 2
/// ```
#[derive(Args, Clone, Debug)]
pub struct BonsaiArgs {
    /// User prompt; no coding-agent system prompt is inserted.
    #[arg(
        long,
        default_value = "Reply with exactly BONSAI_NATIVE_OK and nothing else."
    )]
    pub prompt: String,
    /// Maximum output tokens per request.
    #[arg(long, default_value_t = 64)]
    pub max_tokens: usize,
    /// Sampling temperature; zero is greedy.
    #[arg(long, default_value_t = 0.0)]
    pub temperature: f32,
    /// Repeat in one process with the same resident weights (1..=10).
    #[arg(long, default_value_t = 1)]
    pub repeat: usize,
}
