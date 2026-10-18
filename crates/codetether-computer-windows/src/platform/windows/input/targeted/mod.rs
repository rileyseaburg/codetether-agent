//! Targeted input handlers: focused-field text replacement and
//! client-area clicks.

mod click_client;
mod set_text;
pub use click_client::handle_click_client;
pub use set_text::handle_set_text;
