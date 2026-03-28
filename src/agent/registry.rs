use super::{AgentInfo, AgentMode, AgentRegistry, builtin};

impl AgentRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            agents: std::collections::HashMap::new(),
        }
    }

    /// Register a new agent
    pub fn register(&mut self, info: AgentInfo) {
        self.agents.insert(info.name.clone(), info);
    }

    /// Get agent info by name
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&AgentInfo> {
        self.agents.get(name)
    }

    /// List all agents
    pub fn list(&self) -> Vec<&AgentInfo> {
        self.agents.values().collect()
    }

    /// List primary agents (visible in UI)
    #[allow(dead_code)]
    pub fn list_primary(&self) -> Vec<&AgentInfo> {
        self.agents
            .values()
            .filter(|agent| agent.mode == AgentMode::Primary && !agent.hidden)
            .collect()
    }

    /// Initialize with builtin agents
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        registry.register(builtin::build_agent());
        registry.register(builtin::plan_agent());
        registry.register(builtin::explore_agent());
        registry
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::with_builtins()
    }
}
