//! Static known rate limits per provider/model.
//!
//! Sources: official API docs + empirical observation.
//! RPM = requests per minute, TPM = tokens per minute.

/// Rate limit specification for a model.
#[derive(Debug, Clone, Copy)]
pub(super) struct ModelLimits {
    /// Max requests per minute (0 = unlimited).
    pub rpm: u32,
    /// Max tokens per minute (0 = unlimited).
    pub tpm: u32,
}

impl ModelLimits {
    const fn new(rpm: u32, tpm: u32) -> Self {
        Self { rpm, tpm }
    }
    pub(super) const UNLIMITED: Self = Self { rpm: 0, tpm: 0 };
}

/// Look up limits for `(provider, model)`. Returns `UNLIMITED` if unknown.
pub(super) fn limits_for(provider: &str, model: &str) -> ModelLimits {
    match (provider, model) {
        // Bedrock: conservative defaults (varies by account tier)
        ("bedrock", m) if m.contains("fable") => ModelLimits::new(50, 100_000),
        ("bedrock", m) if m.contains("claude-sonnet-4") => ModelLimits::new(50, 100_000),
        ("bedrock", m) if m.contains("claude-haiku") => ModelLimits::new(200, 400_000),
        ("bedrock", m) if m.contains("claude-opus") => ModelLimits::new(20, 50_000),
        // OpenAI Codex (ChatGPT backend) - empirically observed
        ("openai-codex", "gpt-5.5") => ModelLimits::new(40, 200_000),
        // ZAI / GLM
        ("zai", _) => ModelLimits::new(60, 200_000),
        ("glm5", _) => ModelLimits::new(60, 200_000),
        // Cerebras — very high throughput, minimal throttle
        ("cerebras", _) => ModelLimits::new(300, 1_000_000),
        // Google / Gemini
        ("google", m) if m.contains("flash") => ModelLimits::new(1000, 1_000_000),
        ("google", m) if m.contains("pro") => ModelLimits::new(60, 300_000),
        // MoonshotAI / Kimi
        ("moonshotai", _) => ModelLimits::new(60, 200_000),
        // Everything else: unlimited (let the provider 429 if needed)
        _ => ModelLimits::UNLIMITED,
    }
}
