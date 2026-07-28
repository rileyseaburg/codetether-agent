//! Navigation test module registry.

pub use super::*;

#[path = "navigation_context_tests.rs"]
mod navigation_context_tests;
#[path = "navigation_hash_tests.rs"]
mod navigation_hash_tests;
#[path = "navigation_history_tests.rs"]
mod navigation_history_tests;
#[path = "navigation_lifecycle_js_tests.rs"]
mod navigation_lifecycle_js_tests;
#[path = "navigation_lifecycle_tests.rs"]
mod navigation_lifecycle_tests;
#[path = "navigation_reload_tests.rs"]
mod navigation_reload_tests;
#[path = "navigation_tests.rs"]
mod navigation_tests;
