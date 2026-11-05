//! OAuth for the official Monarch MCP service using CodeTether's secret backend.
//!
//! [`login`] is an interactive auth-command operation. Agent tools use
//! [`access_token`] without seeing passwords or starting browser consent.
//! [`status`] reports record presence; [`logout`] revokes and clears the grant.
//! Backend permissions and the configured namespace determine access. This
//! module neither changes Vault policies nor grants fleet-wide account access.
//! Refreshes use a host-local lock and re-read the current backend record.
mod callback;
mod discovery;
mod oauth;
mod operations;
mod scope;
mod storage;
pub use operations::{access_token, login, logout, status};
