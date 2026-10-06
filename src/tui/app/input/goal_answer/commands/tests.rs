//! Exercise native user commands with checked-out sessions and held reviews.

#[path = "busy_tests.rs"]
mod busy;
mod fixture;
#[path = "hold_tests.rs"]
mod hold;
#[path = "listing_tests.rs"]
mod listing;
#[path = "resume_tests.rs"]
mod resume;
