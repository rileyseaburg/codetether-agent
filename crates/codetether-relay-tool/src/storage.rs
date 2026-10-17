//! storage for relay coordination.
use super::*;

lazy_static::lazy_static! {
    pub(super) static ref RELAY_STORE: RwLock<HashMap<String, RelayRuntimeState>> = RwLock::new(HashMap::new());
    pub(super) static ref AGENT_BUS: OnceCell<Arc<AgentBus>> = OnceCell::const_new();
}

pub(super) async fn get_agent_bus() -> Result<Arc<AgentBus>> {
    let bus = AGENT_BUS
        .get_or_try_init(|| async {
            let bus = AgentBus::new().into_arc();
            Ok::<_, anyhow::Error>(bus)
        })
        .await?;
    Ok(bus.clone())
}
