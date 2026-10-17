//! Opt-in crash capture, local spool storage, and deferred uploads.
//!
//! [initialize] installs panic capture, invokes the caller's memory-guard setup,
//! and uploads pending reports when consent permits. [maybe_prompt_for_consent]
//! provides the existing terminal consent flow. The caller supplies its own
//! application version; report format and retry policy are owned here.
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use codetether_config::Config;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::backtrace::Backtrace;
use std::io::{self, Write};
use std::panic;
use std::path::{Path, PathBuf};
use std::sync::Once;
use std::time::Duration;
use uuid::Uuid;
mod constants;
use constants::*;
include!("wire_capture.rs");
include!("wire_upload.rs");
include!("wire_consent.rs");
#[cfg(test)]
mod tests;
