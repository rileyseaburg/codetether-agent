//! Native resources, prompt policy and execution for a one-shot child.
mod policy;
mod resources;
mod run;
pub use policy::EphemeralPolicy;
pub use resources::EphemeralResources;
pub use run::EphemeralHost;
