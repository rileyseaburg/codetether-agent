//! Shared decoding for modern and legacy Codex rollout records.
//!
//! [normalize_line] yields a common [CodexJsonlRecord] for both formats.
//! [extract_cwd_from_env_context] recovers missing legacy working directories.
//! Record schemas also support session metadata, turn context and title indexes.
mod cwd;
mod envelope;
mod legacy;
mod meta;
mod records;
pub use cwd::extract_cwd_from_env_context;
pub use envelope::normalize_line;
pub use meta::{is_legacy_meta, legacy_session_meta};
pub use records::{
    CodexJsonlRecord, CodexSessionIndexEntry, CodexSessionMetaPayload, CodexTurnContextPayload,
};
#[cfg(test)]
mod tests;
