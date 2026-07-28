use super::*;

#[path = "typed_array/buffer.rs"]
mod buffer;
#[path = "typed_array/constructor.rs"]
mod constructor;
#[path = "typed_array/number.rs"]
mod number;
#[path = "typed_array/prototype.rs"]
mod prototype;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "Uint8Array".into(),
        constructor::uint8(prototype::object("Uint8Array", 1)),
    );
    window.insert(
        "Uint32Array".into(),
        constructor::uint32(prototype::object("Uint32Array", 4)),
    );
    window.insert(
        "Uint16Array".into(),
        constructor::uint16(prototype::object("Uint16Array", 2)),
    );
    window.insert(
        "Int32Array".into(),
        constructor::int32(prototype::object("Int32Array", 4)),
    );
    window.insert(
        "Float32Array".into(),
        constructor::float32(prototype::object("Float32Array", 4)),
    );
    window.insert("ArrayBuffer".into(), buffer::constructor("ArrayBuffer"));
    window.insert(
        "SharedArrayBuffer".into(),
        buffer::constructor("SharedArrayBuffer"),
    );
}
