//! Session workspace tool composition with prior-context and autonomy controls.
//!
//! Implement [Factory] for application registration hooks, then call
//! [registry_for_cwd] to install the session tools in their original order.
pub mod autonomous;
mod batch;
mod build;
mod factory;
mod file_uri;
pub use build::registry_for_cwd;
pub use factory::Factory;
