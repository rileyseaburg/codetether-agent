//! Cache runtime-compiled native CUDA kernels for the packed Candle operators.
use candle_core::Result;
use candle_core::cuda_backend::cudarc::nvrtc;
type Cache = std::sync::OnceLock<std::result::Result<String, String>>;
static PQ2: Cache = Cache::new();
static ROTATION: Cache = Cache::new();
fn source(cache: &'static Cache, code: &str) -> Result<&'static str> {
    cache
        .get_or_init(|| {
            nvrtc::compile_ptx(code)
                .map(|p| p.to_src())
                .map_err(|e| e.to_string())
        })
        .as_ref()
        .map(String::as_str)
        .map_err(|e| candle_core::Error::Msg(e.clone()))
}
pub(super) fn pq2() -> Result<&'static str> {
    source(&PQ2, include_str!("pq2.cu"))
}
pub(super) fn rotation() -> Result<&'static str> {
    source(&ROTATION, include_str!("hadamard.cu"))
}
