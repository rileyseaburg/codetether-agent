//! Owned RGB pixels, cleared automatically on success and every error path.
use zeroize::Zeroizing;

pub(super) struct Pixels {
    pub(super) bytes: Zeroizing<Vec<u8>>,
    pub(super) width: u32,
    pub(super) height: u32,
}
impl Pixels {
    pub(super) fn new(width: u32, height: u32) -> Self {
        Self {
            bytes: Zeroizing::new(vec![0; width as usize * height as usize * 3]),
            width,
            height,
        }
    }
}
