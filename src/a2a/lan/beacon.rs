//! Beacon payload for UDP LAN A2A discovery.

use serde::{Deserialize, Serialize};

pub const PORT: u16 = 47042;
const MAGIC: &str = "codetether-a2a-v1";

#[derive(Debug, Serialize, Deserialize)]
pub struct Beacon {
    magic: String,
    pub name: String,
    pub url: String,
}

impl Beacon {
    pub fn new(name: String, url: String) -> Self {
        Self {
            magic: MAGIC.to_string(),
            name,
            url,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.magic == MAGIC
    }
}
