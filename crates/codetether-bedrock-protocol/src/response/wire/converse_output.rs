//! ConverseOutput wire shape.
use super::*;

#[derive(Debug, Deserialize)]
pub(crate) struct ConverseOutput {
    pub(crate) message: ConverseMessage,
}
