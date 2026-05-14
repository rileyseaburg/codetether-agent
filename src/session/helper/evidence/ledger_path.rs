use std::path::PathBuf;

pub(crate) fn for_session(session_id: &str) -> PathBuf {
    PathBuf::from(".codetether")
        .join("session-ledgers")
        .join(format!("{session_id}.json"))
}
