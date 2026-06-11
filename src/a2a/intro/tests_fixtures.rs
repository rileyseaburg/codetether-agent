//! Shared fixtures for intro-module unit tests.
//!
//! - [`text_message`] builds an A2A [`Message`] for detector tests.
//! - [`with_temp_data_dir`] points `CODETETHER_DATA_DIR` at a shared
//!   test tempdir for the duration of `f`.

#![allow(unsafe_code)]

use std::sync::{Mutex, OnceLock};

use crate::a2a::types::{Message, MessageRole, Part};

/// Build a minimal user `Message` carrying `text` and no metadata.
pub(crate) fn text_message(text: &str) -> Message {
    Message {
        message_id: "m1".to_string(),
        role: MessageRole::User,
        parts: vec![Part::Text {
            text: text.to_string(),
        }],
        context_id: None,
        task_id: None,
        metadata: std::collections::HashMap::new(),
        extensions: vec![],
    }
}

static SHARED_DATA_DIR: OnceLock<tempfile::TempDir> = OnceLock::new();
static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Serialized env-var override so concurrent tests do not race on
/// `CODETETHER_DATA_DIR` (process-global, read on every ledger call).
pub(crate) fn with_temp_data_dir<F: FnOnce()>(f: F) {
    let _guard = ENV_LOCK.lock().expect("env lock poisoned");
    let prior = std::env::var("CODETETHER_DATA_DIR").ok();
    let tmp = SHARED_DATA_DIR.get_or_init(|| tempfile::tempdir().expect("tempdir"));
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", tmp.path()) };
    f();
    unsafe {
        match prior {
            Some(v) => std::env::set_var("CODETETHER_DATA_DIR", v),
            None => std::env::remove_var("CODETETHER_DATA_DIR"),
        }
    }
}
