//! Windows input handling for computer use.

mod click;
mod double_click;
mod drag;
mod focus;
mod key;
mod right_click;
mod scroll;
mod text;
mod validate;
mod wait;

pub use click::handle_click;
pub use double_click::handle_double_click;
pub use drag::handle_drag;
pub use focus::handle_bring_to_front;
pub use key::handle_press_key;
pub use right_click::handle_right_click;
pub use scroll::handle_scroll;
pub use text::handle_type_text;
pub use wait::handle_wait_ms;

pub fn handle_stop() -> anyhow::Result<crate::tool::ToolResult> {
    Ok(super::response::success_result(
        serde_json::json!({"stopped": true}),
    ))
}
