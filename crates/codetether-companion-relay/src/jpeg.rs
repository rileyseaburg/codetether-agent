//! JPEG framing and dimension checks without decoding or saving pixels.

fn be16(bytes: &[u8], at: usize) -> usize {
    usize::from(bytes[at]) << 8 | usize::from(bytes[at + 1])
}
/// Whether `bytes` is a framed JPEG whose SOF0/SOF2 is within 1920×1920.
pub(crate) fn bounded(bytes: &[u8]) -> bool {
    let n = bytes.len();
    if n < 4 || bytes[..2] != [0xff, 0xd8] || bytes[n - 2..] != [0xff, 0xd9] {
        return false;
    }
    let mut offset = 2;
    while offset + 9 < n {
        if bytes[offset] != 0xff {
            return false;
        }
        let marker = bytes[offset + 1];
        let size = be16(bytes, offset + 2);
        if size < 2 || offset + 2 + size > n {
            return false;
        }
        if marker == 0xc0 || marker == 0xc2 {
            let height = be16(bytes, offset + 5);
            let width = be16(bytes, offset + 7);
            return width > 0 && height > 0 && width <= 1920 && height <= 1920;
        }
        offset += size + 2;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::bounded;
    fn jpeg(width: u16, height: u16) -> Vec<u8> {
        let mut bytes = vec![0xff, 0xd8, 0xff, 0xc0, 0, 11, 8];
        bytes.extend(height.to_be_bytes());
        bytes.extend(width.to_be_bytes());
        bytes.extend([1, 1, 0x11, 0, 0xff, 0xd9]);
        bytes
    }
    #[test]
    fn accepts_bounded_and_rejects_oversized() {
        assert!(bounded(&jpeg(1920, 1080)));
        assert!(!bounded(&jpeg(1921, 10)));
        assert!(!bounded(&jpeg(0, 10)));
        assert!(!bounded(b"not a jpeg at all, nope"));
    }
}
