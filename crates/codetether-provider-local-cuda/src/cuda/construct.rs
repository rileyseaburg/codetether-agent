//! Local CUDA construct operations.
use super::*;
impl<R: Runtime> LocalCudaProvider<R> {
    /// Create a new LocalCudaProvider
    ///
    /// # Arguments
    /// * `model_name` — Local model identity.
    ///
    /// # Returns
    /// A provider with an initialized CUDA device and a lazy model runtime.
    ///
    /// # Errors
    /// Returns the CUDA device initialization failure.
    pub fn new(model_name: String) -> Result<Self> {
        let ordinal = parse_env_usize(&["LOCAL_CUDA_ORDINAL", "CODETETHER_LOCAL_CUDA_ORDINAL"], 0);
        let device = Device::new_cuda(ordinal).map_err(|e| {
            anyhow!(
                "CUDA device unavailable for local_cuda (ordinal {}): {}. \
                 Verify `nvidia-smi`, CUDA runtime, and LOCAL_CUDA_ORDINAL.",
                ordinal,
                e
            )
        })?;
        tracing::info!(
            cuda_ordinal = ordinal,
            "Using CUDA device for local inference"
        );

        Ok(Self {
            model_name,
            _device: device,
            model_cache: None,
            runtime: OnceCell::const_new(),
        })
    }

    /// Create with explicit model path
    ///
    /// # Arguments
    /// * `model_name` — Local model identity.
    /// * `model_path` — Model weights path.
    ///
    /// # Returns
    /// A provider using the explicit weights path.
    ///
    /// # Errors
    /// Returns the CUDA device initialization failure.
    pub fn with_model(model_name: String, model_path: String) -> Result<Self> {
        Self::with_paths(model_name, model_path, None, None)
    }

    /// Create with explicit model and tokenizer paths.
    ///
    /// # Arguments
    /// * `model_name` — Local model identity.
    /// * `model_path` — Model weights path.
    /// * `tokenizer_path` — Optional tokenizer override.
    /// * `architecture` — Optional architecture override.
    ///
    /// # Returns
    /// A provider retaining the supplied model configuration.
    ///
    /// # Errors
    /// Returns the CUDA device initialization failure.
    pub fn with_paths(
        model_name: String,
        model_path: String,
        tokenizer_path: Option<String>,
        architecture: Option<String>,
    ) -> Result<Self> {
        let mut provider = Self::new(model_name)?;
        provider.model_cache = Some(ModelCache {
            model_path,
            tokenizer_path,
            architecture,
        });
        Ok(provider)
    }

    /// Check if CUDA is available
    pub fn is_cuda_available() -> bool {
        Device::new_cuda(0).is_ok()
    }

    /// Get device info
    pub fn device_info() -> String {
        match Device::new_cuda(0) {
            Ok(d) => format!("CUDA: {:?}", d),
            Err(_) => "CPU only".to_string(),
        }
    }
}
