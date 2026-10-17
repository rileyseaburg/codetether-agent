//! GLM-5 FP8 provider for Vast.ai serverless deployments
//!
//! This provider connects to a self-hosted vLLM endpoint running GLM-5 FP8
//! on Vast.ai serverless infrastructure. The endpoint exposes an OpenAI-compatible
//! chat completions API (vLLM default).
//!
//! ## Model: GLM-5-FP8
//!
//! - **Architecture**: 744B parameter Mixture-of-Experts (MoE)
//! - **Active Parameters**: 40B per forward pass
//! - **Quantization**: FP8 for efficient inference
//! - **Hardware**: 8x A100 SXM4 80GB
//! - **Features**: MTP speculative decoding enabled
//!
//! ## Configuration (Vault)
//!
//! Store under `secret/data/codetether/providers/glm5`:
//! ```json
//! {
//!   "api_key": "<vast-endpoint-api-key>",
//!   "base_url": "https://route.vast.ai/<endpoint-id>/<api-key>/v1",
//!   "extra": {
//!     "model_name": "glm-5-fp8"
//!   }
//! }
//! ```
//!
//! ## Model reference format
//!
//! Use `glm5/glm-5-fp8`, `glm5/glm-5`, or just `glm-5-fp8` as the model string.
//!
//! ## Environment variable fallback
//!
//! - `GLM5_API_KEY` — API key for the Vast.ai endpoint
//! - `GLM5_BASE_URL` — Base URL of the vLLM endpoint (required)
//! - `GLM5_MODEL` — Model name override (default: glm-5-fp8)

//!
//! Implementation lives in the dedicated workspace provider crate.
pub use codetether_provider_glm5::{DEFAULT_MODEL, Glm5Provider};
