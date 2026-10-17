//! Transfer running peer tasks into an owning lifecycle handle.
use super::*;

impl A2APeerHandle {
    /// Own an already-started endpoint and its discovery tasks.
    ///
    /// # Arguments
    ///
    /// Supply the advertised name, bind address and public URL, the server and
    /// seed-discovery tasks, optional intake task, LAN tasks and mDNS handle.
    /// The caller activates the local identity before transferring ownership.
    ///
    /// # Returns
    ///
    /// A handle that aborts the supplied tasks and deactivates the name on drop.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_a2a_http::peer_handle::A2APeerHandle;
    /// let _construct = A2APeerHandle::new;
    /// ```
    pub fn new(
        agent_name: String,
        bind_addr: String,
        public_url: String,
        server_task: JoinHandle<()>,
        discovery_task: JoinHandle<()>,
        mdns_intake_task: Option<JoinHandle<()>>,
        lan_tasks: Vec<JoinHandle<()>>,
        mdns_handle: Option<mdns::MdnsHandle>,
    ) -> Self {
        Self {
            agent_name,
            bind_addr,
            public_url,
            server_task,
            discovery_task,
            mdns_intake_task,
            lan_tasks,
            _mdns_handle: mdns_handle,
        }
    }
}
