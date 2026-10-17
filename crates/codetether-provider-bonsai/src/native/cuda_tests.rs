//! Explicit GPU parity gate for native PQ2/Hadamard kernels. Never runs by default.
use candle_core::{Device, Tensor};
#[test]
#[ignore = "requires an isolated CUDA runner and NVRTC; not a CPU-only test"]
fn native_pq2_cuda_matches_cpu_reference() {
    let gpu = Device::new_cuda(0).unwrap();
    let cpu = Device::Cpu;
    let mut bytes = vec![0u8; 68];
    bytes[0..2].copy_from_slice(&0x3c00u16.to_le_bytes());
    bytes[2..34].fill(0xe4);
    bytes[34..36].copy_from_slice(&0x3800u16.to_le_bytes());
    bytes[36..].fill(0x1b);
    let x = Tensor::from_vec(
        (0..256).map(|i| (i % 7) as f32 - 3.).collect::<Vec<_>>(),
        (2, 128),
        &cpu,
    )
    .unwrap();
    let w = Tensor::from_slice(&bytes, bytes.len(), &cpu).unwrap();
    let op = super::matmul::Pq2 {
        rows: 2,
        columns: 128,
    };
    let expected = op
        .apply(&x, &w)
        .unwrap()
        .flatten_all()
        .unwrap()
        .to_vec1::<f32>()
        .unwrap();
    let actual = op
        .apply(&x.to_device(&gpu).unwrap(), &w.to_device(&gpu).unwrap())
        .unwrap()
        .flatten_all()
        .unwrap()
        .to_vec1::<f32>()
        .unwrap();
    for (a, b) in actual.iter().zip(expected) {
        assert!((a - b).abs() < 1e-4);
    }
}
