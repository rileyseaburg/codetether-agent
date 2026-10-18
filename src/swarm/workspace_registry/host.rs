//! Native rooted workspace tools with the application's catalog.
pub(crate) type Native =
    codetether_native_swarm::workspace::WorkspaceHost<crate::tool::registry_lifecycle::Native>;
