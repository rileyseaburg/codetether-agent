//! Evidence scope ledgers, best-effort persistence and writeback summaries.
//!
//! [ledger::build] derives a ledger from provider messages; [ledger_persist::save]
//! and [writeback_persist::save] retain the existing on-disk formats. Callers own
//! final-answer gating and memory-palace synchronization.
pub mod final_note;
pub mod ledger;
pub mod ledger_path;
pub mod ledger_persist;
pub mod message_text;
pub mod scope_extract;
pub mod scope_item;
mod writeback_items;
pub mod writeback_path;
pub mod writeback_persist;
pub mod writeback_read;
