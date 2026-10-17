//! Injected Kubernetes transport fixture.
mod data;
mod lease;
mod response;
mod transport;
use crate::{K8sManager, coordination};
pub use data::State;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
pub fn manager(state: State) -> (K8sManager, Arc<Mutex<State>>) {
    let state = Arc::new(Mutex::new(state));
    let manager = K8sManager {
        client: Some(transport::client(state.clone())),
        namespace: "fixture".into(),
        pod_name: Some("self".into()),
        deployment_name: Some("controller".into()),
        actions: Arc::new(RwLock::new(Vec::new())),
        coordinator: coordination::MutationCoordinator,
    };
    (manager, state)
}
