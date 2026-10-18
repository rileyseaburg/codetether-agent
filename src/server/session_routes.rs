//! Native storage, goal and fork bindings for shared session HTTP routes.
#[path = "session_routes/host.rs"]
mod host;

#[cfg(test)]
#[path = "session_routes/fork.rs"]
mod fork;
#[path = "session_routes/fork_host.rs"]
mod fork_host;
#[path = "session_routes/goal_host.rs"]
mod goal_host;
