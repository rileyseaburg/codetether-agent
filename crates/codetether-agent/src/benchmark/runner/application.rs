//! Native provider, Ralph runtime and version choices for benchmark execution.
/// Application version recorded in benchmark reports.
pub struct PackageVersion;
impl codetether_benchmark_runner::binding::Version for PackageVersion {
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}
/// Native services selected by the public benchmark runner alias.
pub type Native = codetether_benchmark_runner::binding::ApplicationHost<
    crate::provider::registry::application::Native,
    crate::ralph::runtime_host::Native,
    PackageVersion,
>;
