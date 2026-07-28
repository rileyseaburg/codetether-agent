//! Same-origin and security-policy metadata for agent pages.
//!
//! The module models deterministic origin decisions for the native agent
//! browser. It records policy intent and request metadata without performing
//! broad network enforcement.

mod context;
mod origin;
mod page;
mod policy;
mod request;
mod resolve;
mod sandbox;
mod url;

#[cfg(test)]
mod tests;

pub use origin::Origin;
pub use policy::SecurityPolicy;
pub use request::RequestSecurityMetadata;
pub use sandbox::SandboxFlags;
