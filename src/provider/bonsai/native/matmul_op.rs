//! Candle custom-op dispatch for packed PQ2_0 multiplication.
use super::matmul::Pq2;
use candle_core::{CpuStorage, CustomOp2, Layout, Result, Shape};
impl CustomOp2 for Pq2 {
    fn name(&self) -> &'static str {
        "bonsai-pq2-matmul"
    }
    fn cpu_fwd(
        &self,
        a: &CpuStorage,
        al: &Layout,
        w: &CpuStorage,
        wl: &Layout,
    ) -> Result<(CpuStorage, Shape)> {
        super::matmul_cpu::forward(self, a, al, w, wl)
    }
    #[cfg(feature = "candle-cuda")]
    fn cuda_fwd(
        &self,
        a: &candle_core::CudaStorage,
        al: &Layout,
        w: &candle_core::CudaStorage,
        wl: &Layout,
    ) -> Result<(candle_core::CudaStorage, Shape)> {
        super::matmul_cuda::forward(self, a, al, w, wl)
    }
}
