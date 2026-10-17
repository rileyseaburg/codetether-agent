//! Numerical reference tests for decay, delta correction and Prism head ordering.
use candle_core::{Device, Tensor};
#[test]
fn gdn_delta_update_is_not_plain_attention() {
    let d = Device::Cpu;
    let state = Tensor::zeros((1, 2, 2), candle_core::DType::F32, &d).unwrap();
    let q = Tensor::new(&[[1f32, 0.]], &d).unwrap();
    let v = Tensor::new(&[[2f32, 4.]], &d).unwrap();
    let decay = Tensor::new(&[0f32], &d).unwrap();
    let beta = Tensor::new(&[0.5f32], &d).unwrap();
    let (out, next) = super::gdn::step(&state, &q, &q, &v, &decay, &beta).unwrap();
    assert_eq!(
        next.flatten_all().unwrap().to_vec1::<f32>().unwrap(),
        [1., 2., 0., 0.]
    );
    let values = out.flatten_all().unwrap().to_vec1::<f32>().unwrap();
    assert!((values[0] - 1.0 / 2f32.sqrt()).abs() < 1e-6);
}
#[test]
fn gdn_value_head_order_matches_prism_permutation() {
    let x = Tensor::new(&[[0f32, 1., 2., 3., 4., 5.]], &Device::Cpu).unwrap();
    let y = super::gdn_layout::grouped(&x, 2, 3, 1).unwrap();
    assert_eq!(y.to_vec2::<f32>().unwrap()[0], [0., 2., 4., 1., 3., 5.]);
}
