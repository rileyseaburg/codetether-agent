//! ConverseMessage wire shape.
use super::*;

#[derive(Debug, Deserialize)]
pub(crate) struct ConverseMessage {
    #[allow(dead_code)]
    pub(crate) role: String,
    pub(crate) content: Vec<ConverseContent>,
}
