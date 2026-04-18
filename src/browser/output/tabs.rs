use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TabInfo {
    pub index: usize,
    pub url: String,
    pub title: String,
    pub active: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TabList {
    pub tabs: Vec<TabInfo>,
}
