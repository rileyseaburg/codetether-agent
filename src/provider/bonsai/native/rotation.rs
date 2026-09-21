//! Candle Hadamard reference operator with explicit forward/inverse ordering.
use candle_core::{CpuStorage, CustomOp2, Layout, Result, Shape, Tensor};
pub(super) struct Rotation {
    pub inverse: bool,
}
impl Rotation {
    pub fn apply(&self, input: &Tensor, signs: &Tensor) -> Result<Tensor> {
        input
            .contiguous()?
            .apply_op2_no_bwd(&signs.contiguous()?, self)
    }
}
impl CustomOp2 for Rotation {
    fn name(&self) -> &'static str {
        "bonsai-hadamard"
    }
    fn cpu_fwd(
        &self,
        input: &CpuStorage,
        il: &Layout,
        signs: &CpuStorage,
        sl: &Layout,
    ) -> Result<(CpuStorage, Shape)> {
        super::rotation_cpu::forward(self.inverse, input, il, signs, sl)
    }
    #[cfg(feature = "candle-cuda")]
    fn cuda_fwd(
        &self,
        input: &candle_core::CudaStorage,
        il: &Layout,
        signs: &candle_core::CudaStorage,
        sl: &Layout,
    ) -> Result<(candle_core::CudaStorage, Shape)> {
        super::rotation_cuda::forward(self.inverse, input, il, signs, sl)
    }
}
