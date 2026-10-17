//! Kubernetes login configuration retained for token refresh.

#[derive(Clone, Debug)]
pub(crate) struct KubernetesAuthConfig {
    pub(crate) address: String,
    pub(crate) role: String,
    pub(crate) auth_mount: String,
    pub(crate) jwt_path: String,
}
