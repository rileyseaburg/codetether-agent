//! Skills advertised by the application default card.
use super::*;
pub(super) fn skills() -> Vec<AgentSkill> {
    vec![
        AgentSkill {
            id: "code".to_string(),
            name: "Code Generation".to_string(),
            description: "Write, edit, and refactor code".to_string(),
            tags: vec!["code".to_string(), "programming".to_string()],
            examples: vec![
                "Write a function to parse JSON".to_string(),
                "Refactor this code to use async/await".to_string(),
            ],
            input_modes: vec!["text/plain".to_string()],
            output_modes: vec!["text/plain".to_string()],
        },
        AgentSkill {
            id: "debug".to_string(),
            name: "Debugging".to_string(),
            description: "Debug and fix code issues".to_string(),
            tags: vec!["debug".to_string(), "fix".to_string()],
            examples: vec![
                "Why is this function returning undefined?".to_string(),
                "Fix the null pointer exception".to_string(),
            ],
            input_modes: vec!["text/plain".to_string()],
            output_modes: vec!["text/plain".to_string()],
        },
        AgentSkill {
            id: "explain".to_string(),
            name: "Code Explanation".to_string(),
            description: "Explain code and concepts".to_string(),
            tags: vec!["explain".to_string(), "learn".to_string()],
            examples: vec![
                "Explain how this algorithm works".to_string(),
                "What does this regex do?".to_string(),
            ],
            input_modes: vec!["text/plain".to_string()],
            output_modes: vec!["text/plain".to_string()],
        },
    ]
}
