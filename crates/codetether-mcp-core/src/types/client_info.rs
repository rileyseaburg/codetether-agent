//! ClientInfo wire contract and construction.
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}
