//! Blender Python code for selecting and framing objects.

use std::path::Path;

pub fn select_frame_code(name: &str, output: &Path) -> String {
    format!(
        "import bpy,fnmatch,json,pathlib\n{}\n{}\n{}\npathlib.Path(r'{}').write_text(json.dumps(state))",
        setup(name),
        select_body(),
        view_body(),
        output.display()
    )
}

fn setup(name: &str) -> String {
    format!("name={name:?}\npat=name if any(c in name for c in '*?[]') else '*'+name+'*'")
}

fn select_body() -> &'static str {
    "matches=[o for o in bpy.data.objects if o.name==name or fnmatch.fnmatch(o.name,pat)]\n\
bpy.ops.object.select_all(action='DESELECT')\n\
for o in matches:\n    o.hide_set(False); o.hide_viewport=False; o.select_set(True)\n\
if matches:\n    bpy.context.view_layer.objects.active=matches[0]"
}

fn view_body() -> &'static str {
    "framed=False\n\
for a in bpy.context.screen.areas:\n    if a.type=='VIEW_3D':\n\
        r=next((r for r in a.regions if r.type=='WINDOW'),None)\n        if r:\n\
            with bpy.context.temp_override(area=a,region=r):\n\
                bpy.ops.view3d.view_selected(use_all_regions=False)\n                framed=True\n\
            break\n\
state={'available':True,'matched_names':[o.name for o in matches],\
'selected_object_names':[o.name for o in bpy.context.selected_objects],\
'active_object_name':bpy.context.object.name if bpy.context.object else None,\
'framed':framed,'selection_method':'python_console'}"
}
