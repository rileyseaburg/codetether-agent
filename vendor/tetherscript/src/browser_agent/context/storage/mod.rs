//! Browser context storage-state and page sessionStorage helpers.

#[path = "clear.rs"]
mod clear;
#[path = "context_api.rs"]
mod context_api;
#[path = "local.rs"]
mod local;
#[path = "origin.rs"]
mod origin;
#[path = "page_api.rs"]
mod page_api;
#[path = "sync.rs"]
mod sync;

#[cfg(test)]
#[path = "tests_clear.rs"]
mod tests_clear;
#[cfg(test)]
#[path = "tests_cookie.rs"]
mod tests_cookie;
#[cfg(test)]
#[path = "tests_session.rs"]
mod tests_session;
#[cfg(test)]
#[path = "tests_state.rs"]
mod tests_state;
