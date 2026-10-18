//! Test-only module namespace.
use super::session_loader;

include!("tests.rs");

#[path = "session_loader_real_tests.rs"]
mod session_loader_real_tests;
