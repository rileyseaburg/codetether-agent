//! Shared private mux discovery and registry persistence.
pub(super) use codetether_mux_registry::{
    MuxRecord, SessionTarget, find_session, find_workspace, for_workspace, list, load, load_key,
    remove_key, store, validate_name,
};
