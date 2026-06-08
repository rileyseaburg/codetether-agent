//! Windows input handling for computer use.

mod blender;
mod blender_clipboard;
mod blender_console;
mod blender_evidence;
mod blender_focus;
mod blender_frame;
mod blender_pattern;
mod blender_query;
mod blender_query_script;
mod blender_select;
mod blender_select_meta;
mod blender_select_python;
mod blender_select_result;
mod blender_select_ui;
mod blender_sequence;
mod blender_state_file;
mod blender_timing;
mod blender_view_script;
mod click;
mod double_click;
mod drag;
mod focus;
mod key;
mod modifiers;
mod mouse;
mod mouse_result;
mod report;
mod right_click;
mod scroll;
mod targeted;
mod text;
mod validate;
mod wait;

pub use blender::{handle_blender_select_frame, handle_focus_viewport};
pub use click::handle_click;
pub use double_click::handle_double_click;
pub use drag::handle_drag;
pub use focus::handle_bring_to_front;
pub use key::handle_press_key;
pub use mouse::{handle_mouse_down, handle_mouse_move, handle_mouse_up};
pub use right_click::handle_right_click;
pub use scroll::handle_scroll;
pub use targeted::{handle_click_client, handle_set_text};
pub use text::handle_type_text;
pub use wait::handle_wait_ms;

pub fn handle_stop() -> anyhow::Result<crate::tool::ToolResult> {
    Ok(super::response::success_result(
        serde_json::json!({"stopped": true}),
    ))
}
