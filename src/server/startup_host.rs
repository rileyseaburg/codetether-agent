//! Application types and identity used by native server startup routing.
type Tools = crate::tool::registry_lifecycle::Native;
type Routing =
    codetether_native_server::ApplicationRoutes<Tools, crate::session::Session, Identity>;
pub(super) type AppHost = codetether_native_server::Host<Tools, Routing, { cfg!(test) }>;
pub(super) struct Identity;
impl codetether_native_server::PackageIdentity for Identity {
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    fn name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }
}
