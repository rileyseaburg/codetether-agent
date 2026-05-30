//! Python snippets for querying Blender session state.

use std::path::PathBuf;

const OUT: &str = "codetether_blender_selected.json";
const QUERY: &str = "'selected_object_names':[o.name for o in bpy.context.selected_objects],\
'active_object_name':bpy.context.object.name if bpy.context.object else None";

pub fn output_path() -> PathBuf {
    std::env::temp_dir().join(OUT)
}

pub fn command() -> String {
    let path = output_path().display().to_string();
    let code = format!("import bpy,json,pathlib;{}", writer(&path));
    format!("exec({code:?})")
}

fn writer(path: &str) -> String {
    format!("pathlib.Path(r'{path}').write_text(json.dumps({{{QUERY}}}))")
}
