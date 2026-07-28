//! Test module registry for browser agent slices.

pub use super::*;

#[path = "actionability_tests.rs"]
mod actionability_tests;
#[path = "clipboard_tests.rs"]
mod clipboard_tests;
#[path = "context_state_tests.rs"]
mod context_state_tests;
#[path = "dialog_tests.rs"]
mod dialog_tests;
#[path = "events_tests.rs"]
mod events_tests;
#[path = "focus_tests.rs"]
mod focus_tests;
#[path = "hit_tests.rs"]
mod hit_tests;
#[path = "keyboard_tests.rs"]
mod keyboard_tests;
#[path = "limits_tests.rs"]
mod limits_tests;
#[path = "locator_attr_tests.rs"]
mod locator_attr_tests;
#[path = "locator_label_tests.rs"]
mod locator_label_tests;
#[path = "locator_tests.rs"]
mod locator_tests;
#[path = "media_tests.rs"]
mod media_tests;
#[path = "navigation_test_modules.rs"]
mod navigation_test_modules;
#[path = "persistent_tests.rs"]
mod persistent_tests;
#[path = "pointer_tests.rs"]
mod pointer_tests;
#[path = "screenshot_element_tests.rs"]
mod screenshot_element_tests;
#[path = "screenshot_tests.rs"]
mod screenshot_tests;
#[path = "selector_ext_tests.rs"]
mod selector_ext_tests;
#[path = "tests.rs"]
mod tests;
#[path = "trace_tests.rs"]
mod trace_tests;
#[path = "upload_test_modules.rs"]
mod upload_test_modules;
#[path = "viewport_tests.rs"]
mod viewport_tests;
#[path = "wait_tests.rs"]
mod wait_tests;
