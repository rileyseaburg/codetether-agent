//! Mocked-local goal editing through command and keyboard dispatchers.

#[path = "conflict_tests.rs"]
mod conflicts;
mod constraints;
#[path = "../commands/tests/fixture.rs"]
mod fixture;
#[path = "in_place_tests.rs"]
mod in_place;
mod keys;
#[path = "open_tests.rs"]
mod open;
#[path = "paste_tests.rs"]
mod paste;
#[path = "review_tests.rs"]
mod review;
#[path = "safety_tests.rs"]
mod safety;
#[path = "save_tests.rs"]
mod save;
