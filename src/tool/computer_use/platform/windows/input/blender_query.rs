//! Blender state query helpers.

use serde_json::{Value, json};

use super::{blender_query_script, blender_timing, key::press_key_name};

pub fn selected_names(target: (i32, i32)) -> Value {
    match query(target).and_then(|()| read_state()) {
        Ok(value) => value,
        Err(error) => json!({
            "available": false,
            "selected_object_names": [],
            "active_object_name": null,
            "error": error.to_string()
        }),
    }
}

pub fn names_vec(state: &Value) -> Vec<String> {
    state["selected_object_names"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|value| value.as_str().map(str::to_string))
        .collect()
}

fn query(target: (i32, i32)) -> anyhow::Result<()> {
    crate::platform::windows::computer_use::send_click(target.0, target.1)?;
    press_key_name("SHIFT F4")?;
    blender_timing::dialog();
    crate::platform::windows::computer_use::send_text(&blender_query_script::command())?;
    press_key_name("ENTER")?;
    blender_timing::dialog();
    Ok(())
}

fn read_state() -> anyhow::Result<Value> {
    let text = std::fs::read_to_string(blender_query_script::output_path())?;
    let mut value: Value = serde_json::from_str(&text)?;
    value["available"] = json!(true);
    Ok(value)
}
