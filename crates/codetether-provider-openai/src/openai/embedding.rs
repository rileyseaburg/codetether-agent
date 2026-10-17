//! Embedding request and response handling.

use super::*;
mod prefix;
mod request;
mod response;
mod wire;
use prefix::safe_char_prefix;
use response::parse_embedding_response;
use wire::*;
