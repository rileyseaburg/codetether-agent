//! Mocked local Morph fixtures rooted in the workspace; production guards stay enabled.

#[path = "fixtures/workspace.rs"]
mod workspace;

#[path = "morph_integration/environment.rs"]
mod environment;
#[path = "morph_integration/mock.rs"]
mod mock;

#[path = "morph_integration/edit_exact.rs"]
mod edit_exact;
#[path = "morph_integration/edit_flow.rs"]
mod edit_flow;
#[path = "morph_integration/multiedit_exact.rs"]
mod multiedit_exact;
#[path = "morph_integration/multiedit_flow.rs"]
mod multiedit_flow;
#[path = "morph_integration/opt_in.rs"]
mod opt_in;
