//! HTTP capability authority.
//!
//! The capability scopes outgoing requests by origin, method, optional path
//! prefix, and harness-bound headers. TetherScript code receives an opaque
//! capability value; it can invoke or narrow the grant, but cannot inspect
//! bound secret header values.

#[path = "http_cap/authority.rs"]
mod authority;
#[path = "http_cap/bound_header.rs"]
mod bound_header;
#[path = "http_cap/describe.rs"]
mod describe;
#[path = "http_cap/invoke.rs"]
mod invoke;
#[path = "http_cap/narrow.rs"]
mod narrow;
#[path = "http_cap/narrow_methods.rs"]
mod narrow_methods;
#[path = "http_cap/narrow_origins.rs"]
mod narrow_origins;
#[path = "http_cap/narrow_path.rs"]
mod narrow_path;
#[path = "http_cap/request.rs"]
mod request;
#[path = "http_cap/scope.rs"]
mod scope;
#[path = "http_cap/url.rs"]
mod url;

pub use authority::HttpAuthority;
