//! Network route model for agent-controlled pages.
//!
//! This module stores deterministic request interception rules. It does not
//! perform I/O; page or context code can ask a [`RouteTable`] how a request
//! should be handled and then apply the selected [`RouteAction`].

mod action;
mod bridge;
mod log_entry;
mod page;
mod pattern;
mod request;
mod route;
mod security_enforcement;
mod shared;
mod table;

pub use action::{RouteAction, RouteFulfillment};
pub(crate) use bridge::js_route_handler;
pub use pattern::RoutePattern;
pub use request::{NetworkLogEntry, RouteRequest};
pub use route::{NetworkRoute, RouteId, RouteRule};
pub(crate) use shared::{shared_route_table, SharedRouteTable};
pub use table::RouteTable;

#[cfg(test)]
mod tests;
