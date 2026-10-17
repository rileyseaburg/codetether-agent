//! Speech-to-text model catalog and decoder readiness.
//!
//! [STT_MODELS] describes known local GGUF speech models. Use [find] for an
//! individual entry and [runnable] for entries with an available decoder.
//! This catalog is separate from the application text-model catalog.
mod entries;
mod types;
pub use entries::STT_MODELS;
pub use types::{DecoderSupport, SttModel};

/// Look up a speech model by id.
///
/// # Arguments
/// * `id` - Exact catalog model identifier.
///
/// # Returns
/// The static catalog entry if the identifier is known.
///
/// # Examples
///
/// ```rust
/// use codetether_voice_core::stt_catalog::{DecoderSupport, find};
///
/// let m = find("nemotron-speech-streaming-en-0.6b").unwrap();
/// assert_eq!(m.arch, "parakeet");
/// assert_eq!(m.support, DecoderSupport::MissingDecoder);
/// ```
pub fn find(id: &str) -> Option<&'static SttModel> {
    STT_MODELS.iter().find(|model| model.id == id)
}

/// Ids marked as having an in-process decoder in this catalog.
///
/// # Arguments
/// None; reads the built-in catalog.
///
/// # Returns
/// Catalog IDs with Available decoder support, preserving catalog order.
///
/// # Examples
/// ```
/// use codetether_voice_core::stt_catalog::{DecoderSupport, find, runnable};
/// assert!(runnable().iter().all(|id| find(id).unwrap().support == DecoderSupport::Available));
/// ```
pub fn runnable() -> Vec<&'static str> {
    STT_MODELS
        .iter()
        .filter(|m| m.support == DecoderSupport::Available)
        .map(|m| m.id)
        .collect()
}

#[cfg(test)]
mod tests;
