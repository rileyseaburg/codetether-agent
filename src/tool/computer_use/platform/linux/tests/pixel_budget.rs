//! Oversized PNG headers must fail before allocating their advertised pixels.

use super::*;

#[test]
fn oversized_png_dimensions_are_rejected_before_decoding() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("oversized-dimensions.png");
    let mut bytes = png();
    // Rewrite the IHDR dimensions and checksum, retaining a tiny compressed body.
    bytes[16..20].copy_from_slice(&8192u32.to_be_bytes());
    bytes[20..24].copy_from_slice(&4097u32.to_be_bytes());
    let checksum = crc32(&bytes[12..29]);
    bytes[29..33].copy_from_slice(&checksum.to_be_bytes());
    std::fs::write(&path, bytes).unwrap();
    let error = image_file::load(&path).unwrap_err();
    assert!(error.to_string().contains("safety budget"), "{error:#}");
}

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320 & 0u32.wrapping_sub(crc & 1));
        }
    }
    !crc
}
