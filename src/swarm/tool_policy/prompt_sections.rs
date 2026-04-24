pub fn coordination_prompt(read_only: bool, model: &str) -> String {
    if read_only {
        return String::new();
    }
    format!(
        "SMART SPAWN POLICY (mandatory):\n- Any spawned agent MUST use a different model than your current model ('{model}')\n- Spawned model MUST be free/subscription-eligible (e.g. '*:free', openai-codex/*, github-copilot/*, gemini-web/*, local_cuda/*)\n- Include `model` when calling agent.spawn\n\nSHARING RESULTS:\nUse swarm_share to collaborate with other sub-agents:\n- swarm_share({{action: 'publish', key: 'my-finding', value: '...', tags: ['research']}}) to share a result\n- swarm_share({{action: 'get', key: 'some-key'}}) to retrieve a result from another agent\n- swarm_share({{action: 'list'}}) to see all shared results\n- swarm_share({{action: 'query_tags', tags: ['research']}}) to find results by tag"
    )
}

pub fn workflow_prompt(read_only: bool, prd_filename: &str) -> String {
    if read_only {
        return String::new();
    }
    format!(
        "COMPLEX TASKS:\nIf your task is complex and involves multiple implementation steps, use the prd + ralph workflow:\n1. Call prd({{action: 'analyze', task_description: '...'}}) to understand what's needed\n2. Break down into user stories with acceptance criteria\n3. Call prd({{action: 'save', prd_path: '{prd_filename}', project: '...', feature: '...', stories: [...]}})\n4. Call ralph({{action: 'run', prd_path: '{prd_filename}'}}) to execute\n\nNOTE: Use your unique PRD file '{prd_filename}' so parallel agents don't conflict."
    )
}
