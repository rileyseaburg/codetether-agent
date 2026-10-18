//! Active model configuration assembled from derived provider-step state.
//! [ModelState::new] retains the provider and clones derived settings for the
//! prompt loop. The generic registry stays owned by the embedding application.
mod new;
mod state;
pub use state::ModelState;
