//! Blender state JSON file helpers.

use serde_json::Value;

use super::{blender_query_script, blender_timing};

pub fn reset() -> anyhow::Result<()> {
    match std::fs::remove_file(blender_query_script::output_path()) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}

pub fn read_waiting() -> anyhow::Result<Value> {
    for _ in 0..20 {
        if let Ok(value) = read_now() {
            return Ok(value);
        }
        blender_timing::probe();
    }
    let path = blender_query_script::output_path();
    anyhow::bail!("Blender state file was not written: {}", path.display())
}

fn read_now() -> anyhow::Result<Value> {
    let text = std::fs::read_to_string(blender_query_script::output_path())?;
    Ok(serde_json::from_str(&text)?)
}
