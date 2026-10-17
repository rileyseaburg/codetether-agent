//! Reference normalization and activations expressed as Candle tensor operations.
use candle_core::{D, Result, Tensor};
pub(super) fn rms(x: &Tensor, weight: &Tensor, epsilon: f64) -> Result<Tensor> {
    x.broadcast_div(
        &x.sqr()?
            .mean_keepdim(D::Minus1)?
            .affine(1.0, epsilon)?
            .sqrt()?,
    )?
    .broadcast_mul(weight)
}
pub(super) fn l2(x: &Tensor, epsilon: f64) -> Result<Tensor> {
    x.broadcast_div(
        &x.sqr()?
            .sum_keepdim(D::Minus1)?
            .sqrt()?
            .clamp(epsilon, f64::INFINITY)?,
    )
}
pub(super) fn sigmoid(x: &Tensor) -> Result<Tensor> {
    x.neg()?.exp()?.affine(1.0, 1.0)?.recip()
}
pub(super) fn silu(x: &Tensor) -> Result<Tensor> {
    x * sigmoid(x)?
}
pub(super) fn softplus(x: &Tensor) -> Result<Tensor> {
    // Stable log(1+exp(x)) = max(x,0)+log(1+exp(-abs(x))).
    x.clamp(0.0, f64::INFINITY)? + x.abs()?.neg()?.exp()?.affine(1.0, 1.0)?.log()?
}
