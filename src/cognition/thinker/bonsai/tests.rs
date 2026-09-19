//! Native format reference tests; no external inference runtime is involved.
#[test]
fn pq2_bit_order_and_half_scale() {
    let mut block = [0u8; 34];
    block[0..2].copy_from_slice(&0x3c00u16.to_le_bytes());
    block[2..].fill(0b11_10_01_00);
    let values = super::pq2::decode(&block).unwrap();
    for lane in values.chunks_exact(4) {
        assert_eq!(lane, [-1.0, 0.0, 1.0, 2.0]);
    }
    block[0..2].copy_from_slice(&0x3800u16.to_le_bytes());
    assert_eq!(super::pq2::decode(&block).unwrap()[3], 1.0);
    assert!(super::pq2::decode(&block[..33]).is_err());
    block[0..2].copy_from_slice(&0x7c00u16.to_le_bytes());
    assert!(super::pq2::decode(&block).is_err());
}
#[test]
fn hadamard_inverse_uses_reverse_sign_order() {
    let mut values = (0..2048).map(|i| (i % 17) as f32 - 8.0).collect::<Vec<_>>();
    let original = values.clone();
    let signs = (0..2048)
        .map(|i| if i % 3 == 0 { -1.0 } else { 1.0 })
        .collect::<Vec<_>>();
    super::hadamard::transform(&mut values, &signs, false).unwrap();
    super::hadamard::transform(&mut values, &signs, true).unwrap();
    for (actual, expected) in values.iter().zip(original) {
        assert!((actual - expected).abs() < 1e-4);
    }
}
#[test]
fn bounded_header_rejects_truncated_input() {
    let mut input = std::io::Cursor::new(b"GGUF".to_vec());
    assert!(super::Index::read(&mut input).is_err());
}
