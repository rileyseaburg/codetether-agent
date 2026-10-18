//! Scripted Blender select-frame path.

use serde_json::{Value, json};

use super::{blender_console, blender_query_script, blender_state_file, blender_view_script};

pub fn run(target: (i32, i32), name: &str) -> Value {
    match execute(target, name) {
        Ok(value) => value,
        Err(error) => json!({
            "available": false,
            "selection_method": "python_console",
            "selected_object_names": [],
            "active_object_name": null,
            "error": error.to_string()
        }),
    }
}

fn execute(target: (i32, i32), name: &str) -> anyhow::Result<Value> {
    blender_state_file::reset()?;
    let output = blender_query_script::output_path();
    let code = blender_view_script::select_frame_code(name, &output);
    blender_console::exec(target, &code)?;
    let mut value = blender_state_file::read_waiting()?;
    value["available"] = json!(true);
    Ok(value)
}
