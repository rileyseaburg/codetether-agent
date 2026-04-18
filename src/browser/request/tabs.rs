use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TabSelectRequest {
    pub index: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewTabRequest {
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CloseTabRequest {
    pub index: Option<usize>,
}
