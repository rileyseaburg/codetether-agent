//! Typed writeback proposals, rejection evidence and commit events.
//!
//! [WritebackJournal] stages [Op] values under a [TxnId], records rejections,
//! and commits only pending proposals. [JournalEntry] retains the serialized
//! evidence. [append_entries] writes an ordered batch to a caller-resolved path.
mod append;
mod entry;
mod journal;
mod operation;
mod rejection;
mod transaction_id;
pub use append::append_entries;
pub use entry::JournalEntry;
pub use journal::WritebackJournal;
pub use operation::Op;
pub use rejection::RejectReason;
pub use transaction_id::TxnId;
#[cfg(test)]
mod tests;
