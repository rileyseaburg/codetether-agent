//! CUDA launch through Candle/cudarc, retaining the weights in PQ2_0 packing.
use super::matmul::Pq2;
use candle_core::cuda_backend::cudarc::driver::{LaunchConfig, PushKernelArg};
use candle_core::{CudaStorage, Layout, Result, Shape};
pub(super) fn forward(
    op: &Pq2,
    a: &CudaStorage,
    al: &Layout,
    w: &CudaStorage,
    wl: &Layout,
) -> Result<(CudaStorage, Shape)> {
    let shape = op.output(al, wl)?;
    if a.device.id() != w.device.id() {
        candle_core::bail!("PQ2_0 tensors must share a CUDA device");
    }
    let input = a.as_cuda_slice::<f32>()?;
    let weights = w.as_cuda_slice::<u8>()?;
    if input.len() != al.shape().elem_count() || weights.len() != wl.shape().elem_count() {
        candle_core::bail!("PQ2_0 storage mismatch");
    }
    let columns = u32::try_from(op.columns).map_err(|e| candle_core::Error::Msg(e.to_string()))?;
    let rows = u32::try_from(op.rows).map_err(|e| candle_core::Error::Msg(e.to_string()))?;
    let blocks =
        u32::try_from(shape.elem_count()).map_err(|e| candle_core::Error::Msg(e.to_string()))?;
    let ptx = super::kernels::pq2()?;
    let function = a
        .device
        .get_or_load_custom_func("bonsai_pq2", "bonsai_pq2_v1", ptx)?;
    let mut output = a.device.alloc_zeros::<f32>(shape.elem_count())?;
    let mut launch = function.builder();
    launch
        .arg(input)
        .arg(weights)
        .arg(&mut output)
        .arg(&columns)
        .arg(&rows);
    let config = LaunchConfig {
        grid_dim: (blocks, 1, 1),
        block_dim: (256, 1, 1),
        shared_mem_bytes: 0,
    };
    // SAFETY: dimensions and zero-offset contiguous layouts are checked before launch.
    unsafe { launch.launch(config) }.map_err(|e| candle_core::Error::Msg(e.to_string()))?;
    drop(launch);
    Ok((
        CudaStorage::wrap_cuda_slice(output, a.device.clone()),
        shape,
    ))
}
