//! Native type binding for shared Ralph agent execution.
/// Native execution binding exposed through the stage host associated type.
/// # Examples
/// ```rust,no_run
/// type Task = <codetether_agent::ralph::RalphLoop as codetether_ralph_stage::Host>::Task;
/// type Binding = <Task as codetether_ralph_task::Host>::Agent;
/// fn assert_host<H: codetether_ralph_agent::Host>() {}
/// assert_host::<Binding>();
/// ```
pub type Native = codetether_ralph_agent::application::ApplicationAgent<
    crate::tool::registry_lifecycle::Native,
    crate::swarm::executor::LoopHost,
>;
