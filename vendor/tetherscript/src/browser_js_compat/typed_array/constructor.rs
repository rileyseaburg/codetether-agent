use super::*;

#[path = "constructor/build.rs"]
mod build;
#[path = "constructor/byte.rs"]
mod byte;
#[path = "constructor/numeric.rs"]
mod numeric;

pub(super) fn uint8(prototype: JsValue) -> JsValue {
    build::constructor("Uint8Array", 1, prototype, byte::uint8_array)
}

pub(super) fn uint32(prototype: JsValue) -> JsValue {
    build::constructor("Uint32Array", 4, prototype, numeric::uint32_array)
}

pub(super) fn uint16(prototype: JsValue) -> JsValue {
    build::constructor("Uint16Array", 2, prototype, numeric::uint16_array)
}

pub(super) fn int32(prototype: JsValue) -> JsValue {
    build::constructor("Int32Array", 4, prototype, numeric::int32_array)
}

pub(super) fn float32(prototype: JsValue) -> JsValue {
    build::constructor("Float32Array", 4, prototype, numeric::float32_array)
}
