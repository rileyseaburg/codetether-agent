//! Stateless helpers for agent message handling and tool execution.
//!
//! Message and image helpers transform supplied data; registered and invalid
//! tool helpers standardize execution results. The caller retains session
//! mutation, permission checks, registry selection and loop orchestration.
pub mod images;
pub mod invalid;
pub mod messages;
pub mod registered;
