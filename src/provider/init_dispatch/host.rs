//! Bind shared constructors to application identity and feature-selected CUDA.
use crate::provider::{copilot, local_cuda};
pub(in crate::provider) type Native = codetether_provider_factory::application::ApplicationHost<
    copilot::ApplicationIdentity,
    local_cuda::LocalCudaProvider,
>;
