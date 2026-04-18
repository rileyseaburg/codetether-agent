use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct NavigationRequest {
    pub url: String,
    pub wait_until: String,
}
