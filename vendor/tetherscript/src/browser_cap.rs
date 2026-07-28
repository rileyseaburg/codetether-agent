//! Browser capability backed by tetherscript browser action envelopes.
//!
//! The public tetherscript API stays language-owned. This module translates
//! those calls into the browserctl protocol implemented by tetherscript browser
//! runtimes.

#[path = "browser_cap/actions.rs"]
mod actions;
#[path = "browser_cap/args.rs"]
mod args;
#[path = "browser_cap/authority.rs"]
mod authority;
#[path = "browser_cap/bridge_url.rs"]
mod bridge_url;
#[path = "browser_cap/call.rs"]
mod call;
#[path = "browser_cap/invoke.rs"]
mod invoke;
#[path = "browser_cap/invoke_helpers.rs"]
mod invoke_helpers;
#[path = "browser_cap/mapping.rs"]
mod mapping;
#[path = "browser_cap/origin.rs"]
mod origin;
#[path = "browser_cap/response.rs"]
mod response;
#[path = "browser_cap/scope.rs"]
mod scope;
#[path = "browser_cap/scope_apply.rs"]
mod scope_apply;
#[path = "browser_cap/scope_list.rs"]
mod scope_list;
#[path = "browser_cap/scope_narrow.rs"]
mod scope_narrow;
#[path = "browser_cap/trace.rs"]
mod trace;
#[path = "browser_cap/transport.rs"]
mod transport;
#[path = "browser_cap/value.rs"]
mod value;

pub use authority::BrowserAuthority;
