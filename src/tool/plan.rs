//! Plan Tool - Enter/exit planning mode for multi-step reasoning.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};
use super::{Tool, ToolResult};
use std::sync::atomic::{AtomicBool, Ordering};
use parking_lot::RwLock;

static IN_PLAN_MODE: AtomicBool = AtomicBool::new(false);

lazy_static::lazy_static! {
    static ref CURRENT_PLAN: RwLock<Option<Plan>> = RwLock::new(None);
}

#[derive(Debug, Clone)]
struct Plan {
    goal: String,
    steps: Vec<PlanStep>,
    current_step: usize,
}

#[derive(Debug, Clone)]
struct PlanStep {
    description: String,
    completed: bool,
    notes: Option<String>,
}

pub struct PlanEnterTool;
pub struct PlanExitTool;

impl Default for PlanEnterTool {
    fn default() -> Self { Self::new() }
}

impl Default for PlanExitTool {
    fn default() -> Self { Self::new() }
}

impl PlanEnterTool {
    pub fn new() -> Self { Self }
}

impl PlanExitTool {
    pub fn new() -> Self { Self }
}

#[derive(Deserialize)]
struct EnterParams {
    goal: String,
    steps: Vec<String>,
}

#[derive(Deserialize)]
struct ExitParams {
    #[serde(default)]
    summary: Option<String>,
    #[serde(default)]
    step_complete: Option<usize>,
    #[serde(default)]
    notes: Option<String>,
}

#[async_trait]
impl Tool for PlanEnterTool {
    fn id(&self) -> &str { "plan_enter" }
    fn name(&self) -> &str { "Enter Plan Mode" }
    fn description(&self) -> &str { "Enter planning mode with a goal and list of steps. Use before complex multi-step tasks." }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "goal": {"type": "string", "description": "The overall goal to achieve"},
                "steps": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Ordered list of steps to complete the goal"
                }
            },
            "required": ["goal", "steps"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: EnterParams = serde_json::from_value(params).context("Invalid params")?;
        
        if IN_PLAN_MODE.load(Ordering::SeqCst) {
            return Ok(ToolResult::error("Already in plan mode. Exit current plan first."));
        }
        
        if p.steps.is_empty() {
            return Ok(ToolResult::error("At least one step is required"));
        }
        
        let plan = Plan {
            goal: p.goal.clone(),
            steps: p.steps.iter().map(|s| PlanStep {
                description: s.clone(),
                completed: false,
                notes: None,
            }).collect(),
            current_step: 0,
        };
        
        *CURRENT_PLAN.write() = Some(plan.clone());
        IN_PLAN_MODE.store(true, Ordering::SeqCst);
        
        let output = format!(
            "📋 Plan Mode Activated\n\nGoal: {}\n\nSteps:\n{}",
            p.goal,
            p.steps.iter().enumerate().map(|(i, s)| format!("  {}. {}", i + 1, s)).collect::<Vec<_>>().join("\n")
        );
        
        Ok(ToolResult::success(output)
            .with_metadata("step_count", json!(p.steps.len()))
            .with_metadata("current_step", json!(1)))
    }
}

#[async_trait]
impl Tool for PlanExitTool {
    fn id(&self) -> &str { "plan_exit" }
    fn name(&self) -> &str { "Exit Plan Mode" }
    fn description(&self) -> &str { "Exit planning mode. Optionally mark a step as complete or provide a summary." }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "summary": {"type": "string", "description": "Summary of what was accomplished"},
                "step_complete": {"type": "integer", "description": "Mark step number as complete (1-indexed)"},
                "notes": {"type": "string", "description": "Notes for the completed step"}
            }
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: ExitParams = serde_json::from_value(params).unwrap_or(ExitParams {
            summary: None,
            step_complete: None,
            notes: None,
        });
        
        if !IN_PLAN_MODE.load(Ordering::SeqCst) {
            return Ok(ToolResult::error("Not in plan mode"));
        }
        
        let (output, completed_count, total_steps, should_exit) = {
            let mut plan_guard = CURRENT_PLAN.write();
            let plan = plan_guard.as_mut().ok_or_else(|| anyhow::anyhow!("No active plan"))?;
            
            // Mark step complete if specified
            if let Some(step_num) = p.step_complete {
                if step_num > 0 && step_num <= plan.steps.len() {
                    let step = &mut plan.steps[step_num - 1];
                    step.completed = true;
                    step.notes = p.notes.clone();
                    plan.current_step = step_num;
                }
            }
            
            // Build status report
            let completed_count = plan.steps.iter().filter(|s| s.completed).count();
            let total_steps = plan.steps.len();
            let status = plan.steps.iter().enumerate().map(|(i, s)| {
                let icon = if s.completed { "✓" } else { "○" };
                let notes = s.notes.as_ref().map(|n| format!(" [{}]", n)).unwrap_or_default();
                format!("  {} {}. {}{}", icon, i + 1, s.description, notes)
            }).collect::<Vec<_>>().join("\n");
            
            let output = format!(
                "📋 Plan Status\n\nGoal: {}\n\nProgress: {}/{} steps\n\n{}\n\n{}",
                plan.goal,
                completed_count,
                total_steps,
                status,
                p.summary.as_ref().map(|s| format!("Summary: {}", s)).unwrap_or_default()
            );
            
            let should_exit = completed_count == total_steps || p.summary.is_some();
            (output, completed_count, total_steps, should_exit)
        };
        
        // If all steps complete or explicit exit, leave plan mode
        if should_exit {
            IN_PLAN_MODE.store(false, Ordering::SeqCst);
            *CURRENT_PLAN.write() = None;
        }
        
        Ok(ToolResult::success(output)
            .with_metadata("completed", json!(completed_count))
            .with_metadata("total", json!(total_steps)))
    }
}
