//! Short-name aliases for common Bedrock model IDs.
//!
//! Allows users to specify e.g. `claude-sonnet-4` instead of the full
//! `us.anthropic.claude-sonnet-4-20250514-v1:0`. Unknown strings are returned
//! unchanged so full model IDs pass through.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::bedrock::resolve_model_id;
//!
//! assert_eq!(
//!     resolve_model_id("claude-opus-4-6"),
//!     "us.anthropic.claude-opus-4-6-v1"
//! );
//! // Unknown IDs pass through unchanged
//! assert_eq!(resolve_model_id("custom.model-id"), "custom.model-id");
//! ```

/// Resolve a short model alias to the full Bedrock model ID.
///
/// Accepts both human-friendly aliases (`claude-opus-4-6`) and canonical
/// Bedrock inference-profile IDs; unknown inputs are returned unchanged.
///
/// # Arguments
///
/// * `model` — The user-supplied model identifier.
///
/// # Returns
///
/// A `&str` pointing at the canonical Bedrock model ID. When `model` already
/// is a full ID (or an unknown string), the returned slice aliases `model`.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::bedrock::resolve_model_id;
///
/// assert_eq!(resolve_model_id("claude-opus-4-7"), "us.anthropic.claude-opus-4-7");
/// assert_eq!(resolve_model_id("nova-lite"), "amazon.nova-lite-v1:0");
/// let full = "us.anthropic.claude-opus-4-6-v1";
/// assert_eq!(resolve_model_id(full), full);
/// ```
pub fn resolve_model_id(model: &str) -> &str {
    match model {
        // --- Anthropic Claude ---
        "claude-opus-4.7" | "claude-opus-4-7" | "claude-4.7-opus" => "us.anthropic.claude-opus-4-7",
        "claude-opus-4.6" | "claude-opus-4-6" | "claude-4.6-opus" => {
            "us.anthropic.claude-opus-4-6-v1"
        }
        "claude-opus-4.5" | "claude-4.5-opus" => "us.anthropic.claude-opus-4-5-20251101-v1:0",
        "claude-opus-4.1" | "claude-4.1-opus" => "us.anthropic.claude-opus-4-1-20250805-v1:0",
        "claude-opus-4" | "claude-4-opus" => "us.anthropic.claude-opus-4-20250514-v1:0",

        "claude-sonnet-4.6" | "claude-4.6-sonnet" | "claude-sonnet-4-6" => {
            "us.anthropic.claude-sonnet-4-6-v1:0"
        }
        "claude-sonnet-4.5" | "claude-4.5-sonnet" => {
            "us.anthropic.claude-sonnet-4-5-20250929-v1:0"
        }
        "claude-sonnet-4" | "claude-4-sonnet" => "us.anthropic.claude-sonnet-4-20250514-v1:0",
        "claude-haiku-4.5" | "claude-4.5-haiku" => "us.anthropic.claude-haiku-4-5-20251001-v1:0",

        // Handle full IDs without version suffix (common user input)
        "us.anthropic.claude-sonnet-4-6" => "us.anthropic.claude-sonnet-4-6-v1:0",
        "us.anthropic.claude-sonnet-4-5" => "us.anthropic.claude-sonnet-4-5-20250929-v1:0",
        "us.anthropic.claude-sonnet-4" => "us.anthropic.claude-sonnet-4-20250514-v1:0",
        "us.anthropic.claude-opus-4-7" => "us.anthropic.claude-opus-4-7",
        "us.anthropic.claude-opus-4-6" => "us.anthropic.claude-opus-4-6-v1",
        "us.anthropic.claude-opus-4-5" => "us.anthropic.claude-opus-4-5-20251101-v1:0",
        "us.anthropic.claude-opus-4-1" => "us.anthropic.claude-opus-4-1-20250805-v1:0",
        "us.anthropic.claude-opus-4" => "us.anthropic.claude-opus-4-20250514-v1:0",
        "us.anthropic.claude-haiku-4-5" => "us.anthropic.claude-haiku-4-5-20251001-v1:0",

        "claude-3.7-sonnet" | "claude-sonnet-3.7" => "us.anthropic.claude-3-7-sonnet-20250219-v1:0",
        "claude-3.5-sonnet-v2" | "claude-sonnet-3.5-v2" => {
            "us.anthropic.claude-3-5-sonnet-20241022-v2:0"
        }
        "claude-3.5-haiku" | "claude-haiku-3.5" => "us.anthropic.claude-3-5-haiku-20241022-v1:0",
        "claude-3.5-sonnet" | "claude-sonnet-3.5" => "us.anthropic.claude-3-5-sonnet-20240620-v1:0",
        "claude-3-opus" | "claude-opus-3" => "us.anthropic.claude-3-opus-20240229-v1:0",
        "claude-3-haiku" | "claude-haiku-3" => "us.anthropic.claude-3-haiku-20240307-v1:0",
        "claude-3-sonnet" | "claude-sonnet-3" => "us.anthropic.claude-3-sonnet-20240229-v1:0",

        // --- Amazon Nova ---
        "nova-pro" => "amazon.nova-pro-v1:0",
        "nova-lite" => "amazon.nova-lite-v1:0",
        "nova-micro" => "amazon.nova-micro-v1:0",
        "nova-premier" => "us.amazon.nova-premier-v1:0",

        // --- Meta Llama ---
        "llama-4-maverick" | "llama4-maverick" => "us.meta.llama4-maverick-17b-instruct-v1:0",
        "llama-4-scout" | "llama4-scout" => "us.meta.llama4-scout-17b-instruct-v1:0",
        "llama-3.3-70b" | "llama3.3-70b" => "us.meta.llama3-3-70b-instruct-v1:0",
        "llama-3.2-90b" | "llama3.2-90b" => "us.meta.llama3-2-90b-instruct-v1:0",
        "llama-3.2-11b" | "llama3.2-11b" => "us.meta.llama3-2-11b-instruct-v1:0",
        "llama-3.2-3b" | "llama3.2-3b" => "us.meta.llama3-2-3b-instruct-v1:0",
        "llama-3.2-1b" | "llama3.2-1b" => "us.meta.llama3-2-1b-instruct-v1:0",
        "llama-3.1-70b" | "llama3.1-70b" => "us.meta.llama3-1-70b-instruct-v1:0",
        "llama-3.1-8b" | "llama3.1-8b" => "us.meta.llama3-1-8b-instruct-v1:0",
        "llama-3-70b" | "llama3-70b" => "meta.llama3-70b-instruct-v1:0",
        "llama-3-8b" | "llama3-8b" => "meta.llama3-8b-instruct-v1:0",

        // --- Mistral ---
        "mistral-large-3" | "mistral-large" => "mistral.mistral-large-3-675b-instruct",
        "mistral-large-2402" => "mistral.mistral-large-2402-v1:0",
        "mistral-small" => "mistral.mistral-small-2402-v1:0",
        "mixtral-8x7b" => "mistral.mixtral-8x7b-instruct-v0:1",
        "pixtral-large" => "us.mistral.pixtral-large-2502-v1:0",
        "magistral-small" => "mistral.magistral-small-2509",

        // --- DeepSeek ---
        "deepseek-r1" => "us.deepseek.r1-v1:0",
        "deepseek-v3" | "deepseek-v3.2" => "deepseek.v3.2",

        // --- Cohere ---
        "command-r" => "cohere.command-r-v1:0",
        "command-r-plus" => "cohere.command-r-plus-v1:0",

        // --- Qwen ---
        "qwen3-32b" => "qwen.qwen3-32b-v1:0",
        "qwen3-coder" | "qwen3-coder-next" => "qwen.qwen3-coder-next",
        "qwen3-coder-30b" => "qwen.qwen3-coder-30b-a3b-v1:0",

        // --- Google Gemma ---
        "gemma-3-27b" => "google.gemma-3-27b-it",
        "gemma-3-12b" => "google.gemma-3-12b-it",
        "gemma-3-4b" => "google.gemma-3-4b-it",

        // --- Moonshot / Kimi ---
        "kimi-k2" | "kimi-k2-thinking" => "moonshot.kimi-k2-thinking",
        "kimi-k2.5" => "moonshotai.kimi-k2.5",

        // --- AI21 Jamba ---
        "jamba-1.5-large" => "ai21.jamba-1-5-large-v1:0",
        "jamba-1.5-mini" => "ai21.jamba-1-5-mini-v1:0",

        // --- MiniMax ---
        "minimax-m2" => "minimax.minimax-m2",
        "minimax-m2.1" => "minimax.minimax-m2.1",

        // --- NVIDIA ---
        "nemotron-nano-30b" => "nvidia.nemotron-nano-3-30b",
        "nemotron-nano-12b" => "nvidia.nemotron-nano-12b-v2",
        "nemotron-nano-9b" => "nvidia.nemotron-nano-9b-v2",

        // --- Z.AI / GLM ---
        "glm-5" => "zai.glm-5",
        "glm-4.7" => "zai.glm-4.7",
        "glm-4.7-flash" => "zai.glm-4.7-flash",

        // Pass through full model IDs unchanged.
        other => other,
    }
}
