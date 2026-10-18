//! Ralph relay teams with ordered session turns and convergence tracking.
//!
//! Implement [Host] for provider enumeration and native session construction
//! and prompting. [call_relay] owns the shared bus, team registration, handoffs,
//! event forwarding, session restoration and normal team shutdown.
mod host;
mod relay_announce;
mod relay_coordinator;
mod relay_events;
mod relay_finish;
mod relay_prepare;
mod relay_rounds;
mod relay_session;
mod relay_turn;
mod relay_types;
use codetether_bus_core::{
    AgentBus,
    relay::{ProtocolRelayRuntime, RelayAgentProfile},
};
use codetether_context_policy::delegation::DelegationState;
use codetether_ralph_core::events::RalphEvent;
use codetether_session_events::{SessionEvent, SessionResult};
pub use host::Host;
use relay_announce::announce_relay_turn;
pub use relay_coordinator::call_relay_static as call_relay;
use relay_events::forward_relay_events;
use relay_finish::finish_relay;
use relay_prepare::prepare_relay_team;
use relay_rounds::run_relay_rounds;
use relay_session::create_relay_session;
use relay_turn::run_relay_turn;
use relay_types::{PreparedRelayTeam, RelayProgress, RelayRoundContext};
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::sync::mpsc;
use tracing::info;
