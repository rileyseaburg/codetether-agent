//! Blender select-frame step labels.

pub const SELECT_FRAME: &[&str] = &[
    "focus viewport child hwnd when provided, otherwise hwnd",
    "open Python console with Shift+F4",
    "paste exec(script) through clipboard",
    "Python exact/glob object lookup",
    "unhide matched objects",
    "select matches and set active object",
    "view3d.view_selected in first VIEW_3D area override",
    "write selected names, active object, and framed flag to temp JSON",
    "if scripted path fails, recover with F3 Select Pattern and Frame Selected",
    "query Blender state again and report confirmed_selected honestly",
];
