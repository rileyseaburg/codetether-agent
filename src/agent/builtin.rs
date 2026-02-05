//! Built-in agent definitions

use super::{AgentInfo, AgentMode};
use std::path::Path;

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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
///
/// This constant is available for use when creating an explore agent programmatically.
/// Currently used via the `explore_agent()` function which returns the AgentInfo.
#[allow(dead_code)]
pub const EXPLORE_SYSTEM_PROMPT: &str = r#"You are a fast, focused agent for codebase exploration.

Your job is to quickly find relevant code and information. Use tools efficiently:
- Use glob to find files by pattern
- Use grep to search for text
- Use list to see directory contents
- Read files to get details

Be thorough but fast. Return the most relevant results without unnecessary exploration.
"#;

/// Load AGENTS.md from the given directory or any parent directory.
/// Returns the content and path if found.
pub fn load_agents_md(start_dir: &Path) -> Option<(String, std::path::PathBuf)> {
    let mut current = start_dir.to_path_buf();

    loop {
        let agents_path = current.join("AGENTS.md");
        if agents_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&agents_path) {
                return Some((content, agents_path));
            }
        }

        // Try parent directory
        if !current.pop() {
            break;
        }
    }

    None
}

/// Load all AGENTS.md files from the given directory up to the root.
/// Returns a list of (content, path) tuples, from most specific (closest to start_dir) to least.
pub fn load_all_agents_md(start_dir: &Path) -> Vec<(String, std::path::PathBuf)> {
    let mut results = Vec::new();
    let mut current = start_dir.to_path_buf();

    loop {
        let agents_path = current.join("AGENTS.md");
        if agents_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&agents_path) {
                results.push((content, agents_path));
            }
        }

        // Try parent directory
        if !current.pop() {
            break;
        }
    }

    results
}

/// Build a complete system prompt for the build agent, including AGENTS.md content if present.
pub fn build_system_prompt(cwd: &Path) -> String {
    let base_prompt = BUILD_SYSTEM_PROMPT.replace("{cwd}", &cwd.display().to_string());

    // Load AGENTS.md files (closest first)
    let agents_files = load_all_agents_md(cwd);

    if agents_files.is_empty() {
        return base_prompt;
    }

    // Build the AGENTS.md section - include all found files, closest last (takes precedence)
    let mut agents_section = String::new();
    agents_section.push_str("\n\n## Project Instructions (AGENTS.md)\n\n");
    agents_section
        .push_str("The following instructions were loaded from AGENTS.md files in the project.\n");
    agents_section
        .push_str("Follow these project-specific guidelines when working on this codebase.\n\n");

    // Reverse so closest (most specific) comes last and takes precedence
    for (content, path) in agents_files.iter().rev() {
        agents_section.push_str(&format!("### From {}\n\n", path.display()));
        agents_section.push_str(content);
        agents_section.push_str("\n\n");
    }

    format!("{base_prompt}{agents_section}")
}

/// Build a complete system prompt for the plan agent, including AGENTS.md content if present.
#[allow(dead_code)]
pub fn build_plan_system_prompt(cwd: &Path) -> String {
    let base_prompt = PLAN_SYSTEM_PROMPT.replace("{cwd}", &cwd.display().to_string());

    // Load AGENTS.md files
    let agents_files = load_all_agents_md(cwd);

    if agents_files.is_empty() {
        return base_prompt;
    }

    let mut agents_section = String::new();
    agents_section.push_str("\n\n## Project Instructions (AGENTS.md)\n\n");

    for (content, path) in agents_files.iter().rev() {
        agents_section.push_str(&format!("### From {}\n\n", path.display()));
        agents_section.push_str(content);
        agents_section.push_str("\n\n");
    }

    format!("{base_prompt}{agents_section}")
}
