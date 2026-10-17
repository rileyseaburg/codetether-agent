//! Bedrock model IDs used by both request dispatch and provider identity.

pub mod aliases;
pub mod aliases_openai;
mod amazon_meta_mistral;
mod claude;
mod other;
pub use aliases::resolve_model_id;
