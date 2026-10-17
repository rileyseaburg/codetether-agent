//! Compatibility exports for the Anthropic-compatible provider crate.
pub use codetether_provider_anthropic::AnthropicProvider;
pub(crate) use codetether_provider_anthropic::image_block;
#[cfg(test)]
mod cache;
#[cfg(test)]
mod convert {
    pub(super) use codetether_provider_anthropic::encode_messages as messages;
}
