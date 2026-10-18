//! Bounded conversion of Codex rollout archives into provider transcripts.
//!
//! [parse_codex_session_from_path] returns a [ParsedSession] for an application
//! to store. The parser preserves developer messages, reasoning and tool roles,
//! while retaining at most the newest 4,000 messages from large archives.
mod budget;
mod consume;
mod finish;
mod parse;
mod parsed;
mod payloads;
mod response;
mod state;
mod state_default;
mod title;
mod usage;
pub use parse::parse_codex_session_from_path;
pub use parsed::ParsedSession;
