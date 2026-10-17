//! Candle/CUDA Hadamard execution; transforms activations, never full weights.
use candle_core::cuda_backend::cudarc::driver::{LaunchConfig, PushKernelArg};
use candle_core::{CudaStorage, Layout, Result, Shape};
pub(super) fn forward(
    inverse: bool,
    input: &CudaStorage,
    il: &Layout,
    signs: &CudaStorage,
    sl: &Layout,
) -> Result<(CudaStorage, Shape)> {
    let (width, count) = super::rotation_layout::check(il, sl)?;
    if input.device.id() != signs.device.id() {
        candle_core::bail!("Hadamard device mismatch");
    }
    let x = input.as_cuda_slice::<f32>()?;
    let s = signs.as_cuda_slice::<f32>()?;
    if x.len() != count || s.len() != width {
        candle_core::bail!("Hadamard storage mismatch");
    }
    let width = u32::try_from(width).map_err(|e| candle_core::Error::Msg(e.to_string()))?;
    let blocks = u32::try_from(count / 1024).map_err(|e| candle_core::Error::Msg(e.to_string()))?;
    let inverse = u32::from(inverse);
    let ptx = super::kernels::rotation()?;
    let function =
        input
            .device
            .get_or_load_custom_func("bonsai_hadamard", "bonsai_hadamard_v1", ptx)?;
    let mut output = input.device.alloc_zeros::<f32>(count)?;
    let mut launch = function.builder();
    launch
        .arg(x)
        .arg(s)
        .arg(&mut output)
        .arg(&width)
        .arg(&inverse);
    // SAFETY: one 256-thread block per 1024 contiguous elements; bounds validated above.
    unsafe {
        launch.launch(LaunchConfig {
            grid_dim: (blocks, 1, 1),
            block_dim: (256, 1, 1),
            shared_mem_bytes: 0,
        })
    }
    .map_err(|e| candle_core::Error::Msg(e.to_string()))?;
    drop(launch);
    Ok((
        CudaStorage::wrap_cuda_slice(output, input.device.clone()),
        il.shape().clone(),
    ))
}
