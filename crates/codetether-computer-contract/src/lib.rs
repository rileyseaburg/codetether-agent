//! Platform-independent computer-use tool contract.
//!
//! [input] defines requests and OCR validation, [schema] advertises supported
//! parameters, and [response] formats results. Operating-system execution stays
//! with the host, which deserializes [input::ComputerUseInput] before dispatch.
pub mod input;
pub mod response;
pub mod schema;

/// Input-mode and application-selector routing rules.
pub mod routing;

/// Platform-independent Blender selection evidence formatting.
pub mod blender_result;
