//! Main-swarm registry composition with safe coordination extensions.

use super::Capability;
use crate::{
    provider::{Provider, ToolDefinition},
    swarm::result_store::ResultStore,
    tool::ToolRegistry,
};
use std::{path::Path, sync::Arc};

pub(crate) fn for_agent(
    provider: Arc<dyn Provider>,
    model: String,
    root: &Path,
    capability: Capability,
    store: Arc<ResultStore>,
    producer_id: String,
) -> (Arc<ToolRegistry>, Vec<ToolDefinition>) {
    codetether_workspace_tools::composition::for_agent::<super::RegistryHost>(
        provider,
        model,
        root,
        capability,
        store,
        producer_id,
    )
}
