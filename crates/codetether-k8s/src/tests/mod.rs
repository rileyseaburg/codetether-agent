//! Kubernetes manager regression tests.

use crate::K8sManager;
mod k8s_manager_initializes_without_cluster;
mod subagent_pod_name_avoids_prefix_collisions;
mod subagent_pod_name_is_sanitized_and_stable;
