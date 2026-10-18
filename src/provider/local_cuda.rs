//! Native backend binding for the shared local CUDA provider.
pub use codetether_provider_local_cuda::LocalCudaConfig;
/// Local CUDA provider using the application's existing thinker backend.
pub type LocalCudaProvider =
    codetether_provider_local_cuda::LocalCudaProvider<crate::cognition::ThinkerClient>;
