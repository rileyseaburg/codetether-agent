//! Native cognition runtime with explicit registry and state-restoration policy.
/// Cognition runtime selecting native thinker services and caller-supplied tools.
///
/// `EMPTY_STATE` preserves test isolation when true and restores persisted state
/// when false. Tool registries implement the shared cognition dispatch contract.
// The bounds document requirements imposed by the underlying runtime host.
#[allow(type_alias_bounds)]
pub type CognitionRuntime<
    R: codetether_cognition_core::execution::dispatch::Registry + std::fmt::Debug + 'static,
    const EMPTY_STATE: bool,
> = codetether_cognition_runtime::CognitionRuntime<super::runtime_host::Native<R, EMPTY_STATE>>;
