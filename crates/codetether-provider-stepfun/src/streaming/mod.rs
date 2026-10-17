//! Streaming HTTP, data lines, choice/tool events, and event selection.
mod choice;
mod decode;
mod line;
mod select;
mod send;
mod tools;
pub(crate) use decode::decode;
#[cfg(test)]
mod tests;
