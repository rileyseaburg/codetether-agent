//! Native runtime/tool/state choices for the shared startup sequence.
use super::*;
use codetether_plugin_verification::PluginRegistry;
use codetether_server_runtime::Parts;
impl<T: Application, A: Routes<T, EMPTY_STATE>, const EMPTY_STATE: bool>
    codetether_server_runtime::Host for Host<T, A, EMPTY_STATE>
{
    type Runtime = Runtime<T, EMPTY_STATE>;
    type State = State<T, EMPTY_STATE>;
    fn new_runtime() -> Self::Runtime {
        Self::Runtime::new_from_env()
    }
    fn install_tools(runtime: &mut Self::Runtime) -> PluginRegistry {
        ToolRegistry::<T>::install_defaults(|tools| runtime.set_tools(tools))
    }
    fn runtime_enabled(runtime: &Self::Runtime) -> bool {
        runtime.is_enabled()
    }
    fn a2a(address: &str, bus: Arc<AgentBus>) -> (AgentCard, Router, Router) {
        A::a2a(address, bus)
    }
    fn state(parts: Parts<Self::Runtime>) -> Self::State {
        parts.into()
    }
    fn router(state: &Self::State, root: Router, nested: Router) -> Router {
        A::router(state, root, nested)
    }
}
