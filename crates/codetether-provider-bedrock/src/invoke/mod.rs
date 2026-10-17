//! InvokeModel execution, synthetic streaming, and error mapping.
mod complete;
pub mod error;
pub mod retention;
mod stream;
mod stream_parts;
#[cfg(test)]
mod tests;
