//! Pricing response struct.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Pricing {
    pub prompt: String,
    pub completion: String,
}
