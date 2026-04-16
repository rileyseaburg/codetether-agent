//! Integration tests for oracle trace storage.
//!
//! Tests use mock remote backends to verify spool-first
//! persistence, upload-on-success cleanup, and bulk sync.

mod fixtures;
mod persist_tests;
mod sync_tests;
