use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageSnapshot {
    pub url: String,
    pub title: String,
    pub text: String,
    pub viewport: Option<Viewport>,
}
