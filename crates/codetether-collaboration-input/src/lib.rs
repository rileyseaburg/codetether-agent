//! Normalize child-agent text, references and image attachments.
//!
//! Deserialize [InputItem] values and pass them to [prepare_in_workspace].
//! [Prepared] preserves text and [MessageImage] attachments for durable delivery;
//! [parameters] provides the corresponding tool schema.
mod input;
mod item;
mod message;
mod schema;
pub use input::{Prepared, prepare_in_workspace};
pub use item::InputItem;
pub use message::MessageImage;
pub use schema::parameters;
#[cfg(test)]
mod tests;
