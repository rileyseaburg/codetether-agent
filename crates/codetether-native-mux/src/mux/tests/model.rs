//! Shared model fixture for native protocol and server tests.
use crate::mux::{isolation::Isolation, model::MuxSnapshot};
use std::path::PathBuf;

pub(super) fn server() -> MuxSnapshot {
    MuxSnapshot::new("work".into(), PathBuf::from("/repo"), Isolation::Shared)
}
