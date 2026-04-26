//! Windows input handling for computer use.

mod click;
mod key;
mod scroll;
mod text;
mod validate;

pub use click::handle_click;
pub use key::handle_press_key;
pub use scroll::handle_scroll;
pub use text::handle_type_text;

pub fn handle_stop() -> anyhow::Result<crate::tool::ToolResult> {
    Ok(super::response::success_result(
        serde_json::json!({"stopped": true}),
    ))
}
