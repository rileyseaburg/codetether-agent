//! Manager initialization, scoping, observation, and action history.

mod actions;
mod bootstrap;
mod debug;
mod in_cluster;
mod initialize;
mod kubeconfig;
mod namespace;
mod naming;
mod replicas;
mod scope;
mod state;
mod status;
pub use state::K8sManager;
