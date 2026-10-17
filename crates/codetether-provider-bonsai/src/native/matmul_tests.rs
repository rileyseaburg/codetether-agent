//! Packed Candle matmul reference tests; CUDA parity runs only on CUDA CI.
use candle_core::{Device, Tensor};
#[test]
fn pq2_candle_matmul_keeps_group_scales() {
    let device = Device::Cpu;
    let mut packed = vec![0u8; 68];
    packed[0..2].copy_from_slice(&0x3c00u16.to_le_bytes());
    packed[2..34].fill(0xaa);
    packed[34..36].copy_from_slice(&0x3800u16.to_le_bytes());
    packed[36..68].fill(0);
    let weights = Tensor::from_slice(&packed, packed.len(), &device).unwrap();
    let input = Tensor::from_slice(&vec![1f32; 256], (2, 128), &device).unwrap();
    let op = super::matmul::Pq2 {
        rows: 2,
        columns: 128,
    };
    assert_eq!(
        op.apply(&input, &weights)
            .unwrap()
            .to_vec2::<f32>()
            .unwrap(),
        [[128., -64.], [128., -64.]]
    );
}
