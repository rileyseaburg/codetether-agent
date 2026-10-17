//! GPU-only Hadamard parity gate.
use candle_core::{Device, Tensor};
#[test]
#[ignore = "requires an isolated CUDA runner and NVRTC"]
fn native_hadamard_cuda_matches_cpu_reference() {
    let gpu = Device::new_cuda(0).unwrap();
    let cpu = Device::Cpu;
    let x = Tensor::from_vec(
        (0..2048).map(|i| (i % 13) as f32 - 6.).collect::<Vec<_>>(),
        (2, 1024),
        &cpu,
    )
    .unwrap();
    let signs = Tensor::from_vec(
        (0..1024)
            .map(|i| if i % 3 == 0 { -1f32 } else { 1. })
            .collect::<Vec<_>>(),
        1024,
        &cpu,
    )
    .unwrap();
    for inverse in [false, true] {
        let op = super::rotation::Rotation { inverse };
        let expected = op
            .apply(&x, &signs)
            .unwrap()
            .flatten_all()
            .unwrap()
            .to_vec1::<f32>()
            .unwrap();
        let actual = op
            .apply(&x.to_device(&gpu).unwrap(), &signs.to_device(&gpu).unwrap())
            .unwrap()
            .flatten_all()
            .unwrap()
            .to_vec1::<f32>()
            .unwrap();
        for (a, b) in actual.iter().zip(expected) {
            assert!((a - b).abs() < 1e-4);
        }
    }
}
