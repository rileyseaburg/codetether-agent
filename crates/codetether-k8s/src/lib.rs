//! Kubernetes manager for scoped pod and deployment operations.
//!
//! [`K8sManager`] resolves an optional client, observes cluster state, and
//! coordinates mutations with namespace-scoped leases. Launch options and
//! operation results are exposed as serializable data types.

mod coordination;
mod deployment_patch;
mod deployment_restart;
mod deployment_scale;
mod list_pods;
mod list_pods_selector;
mod manager;
mod pod;
mod subagent_name;
mod types;
pub use manager::K8sManager;
pub use types::{DeployAction, K8sStatus, PodInfo, SubagentPodSpec, SubagentPodState};
#[cfg(test)]
mod contract;
#[cfg(test)]
mod tests;
