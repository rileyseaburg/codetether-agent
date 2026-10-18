//! Shared provider fixtures for refresh boundary tests.
mod model;
mod provider;
mod registry;
pub(super) use model::model;
pub(super) use provider::StaticProvider;
pub(super) use registry::Registry;
