//! Task-scoped event sink installation and emission.

use super::*;
use fixture::{capture, isolated_sink};

#[path = "session_event_sink_tests/capture.rs"]
mod capture_support;
#[path = "session_event_sink_tests/fixture.rs"]
mod fixture;
#[path = "session_event_sink_tests/lifecycle.rs"]
mod lifecycle;
#[path = "session_event_sink_tests/reasoning.rs"]
mod reasoning;
#[path = "session_event_sink_tests/redaction.rs"]
mod redaction;
#[path = "session_event_sink_tests/tool_events.rs"]
mod tool_events;
