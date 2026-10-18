//! HTTP service traits for the shared cognition runtime.
//!
//! Each implementation forwards one API responsibility to the existing runtime
//! methods. Application thinker and tool-registry types remain selected by `Host`.
mod beliefs;
mod lifecycle;
mod personas;
mod proposals;
mod workspace;
