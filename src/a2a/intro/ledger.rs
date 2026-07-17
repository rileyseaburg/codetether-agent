//! Persistent ledger of peers already introduced to.
//!
//! The in-memory `discovered` set in the discovery loops forgets on
//! restart, which caused thousands of duplicate intros. This ledger
//! persists introduction identities to `<data_dir>/a2a/introduced.json`.
//! First-party LAN identities include hashes of both process capabilities so
//! recycled ports and other local agents cannot suppress a new introduction.
//!
//! Split across:
//! - [`ledger_path`] — filesystem location
//! - [`ledger_load`] — read into `HashSet`
//! - [`ledger_atomic`] — temp-file rename writer
//! - [`ledger_record`] — atomic append

#[path = "ledger_atomic.rs"]
mod ledger_atomic;
#[path = "ledger_load.rs"]
mod ledger_load;
#[path = "ledger_path.rs"]
mod ledger_path;
#[path = "ledger_record.rs"]
mod ledger_record;

pub use ledger_load::load;
pub use ledger_record::record;

/// Whether `identity` was already introduced (normalized, no trailing `/`).
pub fn contains(identity: &str) -> bool {
    load()
        .unwrap_or_default()
        .contains(identity.trim_end_matches('/'))
}
