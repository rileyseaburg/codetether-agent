//! Built-in agent definitions

use super::{AgentInfo, AgentMode};

/// The default "build" agent - full access for development work
pub fn build_agent() -> AgentInfo {
    AgentInfo {
        name: "build".to_string(),
        description: Some("Full access agent for development work".to_string()),
        mode: AgentMode::Primary,
        native: true,
        hidden: false,
        model: None,
        temperature: None,
        top_p: None,
        max_steps: Some(100),
    }
}

/// The "plan" agent - read-only for analysis and exploration
pub fn plan_agent() -> AgentInfo {
    AgentInfo {
        name: "plan".to_string(),
        description: Some("Read-only agent for analysis and code exploration".to_string()),
        mode: AgentMode::Primary,
        native: true,
        hidden: false,
        model: None,
        temperature: None,
        top_p: None,
        max_steps: Some(50),
    }
}

/// The "explore" agent - fast codebase exploration
pub fn explore_agent() -> AgentInfo {
    AgentInfo {
        name: "explore".to_string(),
        description: Some("Fast agent for exploring codebases".to_string()),
        mode: AgentMode::Subagent,
        native: true,
        hidden: false,
        model: None,
        temperature: None,
        top_p: None,
        max_steps: Some(20),
    }
}

/// System prompt for the build agent
pub const BUILD_SYSTEM_PROMPT: &str = r#"You are an expert AI programming assistant called CodeTether Agent.

You help users with software development tasks including:
- Writing and editing code
- Debugging and fixing issues
- Implementing new features
- Refactoring and improving code quality
- Explaining code and concepts

You have access to tools that let you:
- Read and write files
- Run shell commands
- Search the codebase
- List directories

Always:
- Be concise and helpful
- Show your work by using tools
- Explain what you're doing
- Ask clarifying questions when needed
- Follow best practices for the language/framework being used

Current working directory: {cwd}
"#;

/// System prompt for the plan agent
pub const PLAN_SYSTEM_PROMPT: &str = r#"You are an expert AI assistant for code analysis and planning.

Your role is to:
- Explore and understand codebases
- Analyze code structure and architecture
- Plan changes and refactoring
- Answer questions about the code

You have read-only access to the codebase. You can:
- Read files
- Search the codebase
- List directories
- Run safe commands

You should NOT modify any files. If the user wants changes, explain what should be changed and suggest switching to the build agent.

Current working directory: {cwd}
"#;

/// System prompt for the explore agent
pub const EXPLORE_SYSTEM_PROMPT: &str = r#"You are a fast, focused agent for codebase exploration.

Your job is to quickly find relevant code and information. Use tools efficiently:
- Use glob to find files by pattern
- Use grep to search for text
- Use list to see directory contents
- Read files to get details

Be thorough but fast. Return the most relevant results without unnecessary exploration.
"#;
