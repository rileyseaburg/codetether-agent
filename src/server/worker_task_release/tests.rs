//! Shared fixtures for release outcome and queue/event regression coverage.

#[path = "tests/contender.rs"]
mod contender;
#[path = "tests/events.rs"]
mod events;
#[path = "tests/fixtures.rs"]
mod fixtures;
#[path = "tests/guards.rs"]
mod guards;
#[path = "tests/http_error.rs"]
mod http_error;
#[path = "tests/lifecycle.rs"]
mod lifecycle;
#[path = "tests/messages.rs"]
mod messages;
#[path = "tests/missing.rs"]
mod missing;
#[path = "tests/normalization.rs"]
mod normalization;
#[path = "tests/race.rs"]
mod race;
#[path = "tests/race_assertions.rs"]
mod race_assertions;
#[path = "tests/receipts.rs"]
mod receipts;

use fixtures::{queue, request};
