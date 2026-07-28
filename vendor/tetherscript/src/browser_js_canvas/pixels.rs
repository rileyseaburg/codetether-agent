//! Canvas pixel object helpers.

use super::surface::Surface;
use super::*;

pub(super) fn object(width: usize, height: usize, bytes: Vec<JsValue>, checksum: u64) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("width".into(), JsValue::Number(width as f64));
    obj.insert("height".into(), JsValue::Number(height as f64));
    obj.insert("data".into(), JsValue::Array(Rc::new(RefCell::new(bytes))));
    obj.insert("checksum".into(), JsValue::String(checksum.to_string()));
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

pub(super) fn at(surface: &Surface, x: i64, y: i64) -> [u8; 4] {
    if x < 0 || y < 0 || x >= surface.width as i64 || y >= surface.height as i64 {
        return [0, 0, 0, 0];
    }
    if surface.pixels.is_empty() {
        return [0, 0, 0, 0];
    }
    surface.pixels[y as usize * surface.width as usize + x as usize]
}

pub(super) fn checksum(surface: &Surface) -> u64 {
    surface
        .pixels
        .iter()
        .enumerate()
        .fold(0, |acc, (index, pixel)| {
            let packed = u32::from_be_bytes(*pixel) as u64;
            acc.wrapping_add((index as u64 + 1).wrapping_mul(packed + 1))
        })
}
