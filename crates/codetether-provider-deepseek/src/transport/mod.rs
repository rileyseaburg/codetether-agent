//! HTTP execution and server error conversion.
//!
//! `DeepSeekProvider::complete_non_streaming` sends the request and decodes the
//! response. Error wire types and HTTP error formatting remain separate helpers.
mod complete;
mod error;
mod map_error;
