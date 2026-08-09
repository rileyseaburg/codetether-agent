//! Last-line pairing audit for a fully built Bedrock request body.
//!
//! [`super::convert`] normalizes messages during conversion, but anything that
//! mutates `body["messages"]` afterwards (cache-point insertion, checkpoint
//! replay, native InvokeModel remapping) can still ship an assistant `toolUse`
//! whose `toolResult` is missing. Bedrock rejects that with a 400 `Expected
//! toolResult blocks at messages.N.content for the following Ids: ...`, which
//! is a permanent fault, so the turn dies.
//!
//! [`scan`] finds violations; [`repair`] fixes them before send.

#[path = "audit/pairing_error.rs"]
pub(in crate::provider::bedrock) mod pairing_error;
#[path = "audit/repair.rs"]
mod repair;
#[path = "audit/scan.rs"]
mod scan;

pub(in crate::provider::bedrock) use repair::enforce;
#[cfg(test)]
pub(in crate::provider::bedrock) use scan::unpaired;

#[cfg(test)]
#[path = "audit/tests.rs"]
mod tests;
