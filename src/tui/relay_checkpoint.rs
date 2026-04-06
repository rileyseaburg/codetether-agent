//! Relay checkpoint persistence and OKR approval types

use super::*;

struct RelayCheckpoint {
    /// Original user task
    task: String,
    /// Primary model reference requested by the user
    model_ref: String,
    /// Ordered list of agent names in relay order
    ordered_agents: Vec<String>,
    /// Session IDs for each agent (agent name → session UUID)
    agent_session_ids: HashMap<String, String>,
    /// Agent profiles: (name, system instructions, capabilities)
    agent_profiles: Vec<(String, String, Vec<String>)>,
    /// Current round (1-based)
    round: usize,
    /// Current agent index within the round
    idx: usize,
    /// The baton text to pass to the next agent
    baton: String,
    /// Total turns completed so far
    turns: usize,
    /// Convergence hit count
    convergence_hits: usize,
    /// Dynamic spawn count
    dynamic_spawn_count: usize,
    /// RLM handoff count
    rlm_handoff_count: usize,
    /// Workspace directory
    workspace_dir: PathBuf,
    /// When the relay was started
    started_at: String,
    /// OKR ID this relay is associated with (if any)
    #[serde(default)]
    okr_id: Option<String>,
    /// OKR run ID this relay is associated with (if any)
    #[serde(default)]
    okr_run_id: Option<String>,
    /// Key result progress cursor: map of kr_id -> current value
    #[serde(default)]
    kr_progress: HashMap<String, f64>,
    /// Shared relay context snapshot built from structured RLM deltas
    #[serde(default)]
    shared_context: SharedRelayContext,
    /// Number of RLM context-delta distillations performed so far
    #[serde(default)]
    rlm_context_count: usize,
    /// Round-robin pool for assigning models to new relay participants
    #[serde(default)]
    model_rotation: RelayModelRotation,
    /// Agent model assignments (agent name -> provider/model)
    #[serde(default)]
    agent_models: HashMap<String, String>,
}

impl RelayCheckpoint {
    fn checkpoint_path() -> Option<PathBuf> {
        crate::config::Config::data_dir().map(|d| d.join("relay_checkpoint.json"))
    }

    async fn save(&self) -> Result<()> {
        if let Some(path) = Self::checkpoint_path() {
            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            let content = serde_json::to_string_pretty(self)?;
            tokio::fs::write(&path, content).await?;
            tracing::debug!("Relay checkpoint saved");
        }
        Ok(())
    }

    async fn load() -> Option<Self> {
        let path = Self::checkpoint_path()?;
        let content = tokio::fs::read_to_string(&path).await.ok()?;
        serde_json::from_str(&content).ok()
    }

    async fn load_for_workspace(workspace_dir: &Path) -> Option<Self> {
        let checkpoint = Self::load().await?;
        if same_workspace(&checkpoint.workspace_dir, workspace_dir) {
            Some(checkpoint)
        } else {
            tracing::info!(
                checkpoint_workspace = %checkpoint.workspace_dir.display(),
                active_workspace = %workspace_dir.display(),
                "Ignoring relay checkpoint from different workspace"
            );
            None
        }
    }

    async fn delete() {
        if let Some(path) = Self::checkpoint_path() {
            let _ = tokio::fs::remove_file(&path).await;
            tracing::debug!("Relay checkpoint deleted");
        }
    }
}


fn same_workspace(left: &Path, right: &Path) -> bool {
    let left_norm = left.canonicalize().unwrap_or_else(|_| left.to_path_buf());
    let right_norm = right.canonicalize().unwrap_or_else(|_| right.to_path_buf());
    left_norm == right_norm
}

/// Estimate USD cost from model name and token counts.
/// Uses approximate per-million-token pricing for well-known models.

struct PendingOkrApproval {
    /// The OKR being proposed
    okr: Okr,
    /// The OKR run being proposed
    run: OkrRun,
    /// Optional note when we had to fall back to a template draft
    draft_note: Option<String>,
    /// Original task that triggered the OKR
    task: String,
    /// Agent count for the relay
    agent_count: usize,
    /// Model to use
    model: String,
}

impl PendingOkrApproval {
    /// Create a new pending approval from a task
    fn new(task: String, agent_count: usize, model: String) -> Self {
        let okr_id = Uuid::new_v4();

        let okr = default_relay_okr_template(okr_id, &task);

        // Create the run
        let mut run = OkrRun::new(
            okr_id,
            format!("Run {}", chrono::Local::now().format("%Y-%m-%d %H:%M")),
        );
        let _ = run.submit_for_approval();

        Self {
            okr,
            run,
            draft_note: None,
            task,
            agent_count,
            model,
        }
    }

    /// Create a new pending approval by asking the configured model to draft the OKR.
    /// Falls back to a safe template if providers are unavailable or the response can't be parsed.
    async fn propose(task: String, agent_count: usize, model: String) -> Self {
        let mut pending = Self::new(task, agent_count, model);
        let okr_id = pending.okr.id;
        let registry = crate::provider::ProviderRegistry::from_vault()
            .await
            .ok()
            .map(std::sync::Arc::new);

        let task = pending.task.clone();
        let agent_count = pending.agent_count;
        let model = pending.model.clone();

        let (okr, draft_note) = if let Some(registry) = &registry {
            match plan_okr_draft_with_registry(&task, &model, agent_count, registry).await {
                Some(planned) => (okr_from_planned_draft(okr_id, &task, planned), None),
                None => (
                    default_relay_okr_template(okr_id, &task),
                    Some("(OKR: fallback template — model draft parse failed)".to_string()),
                ),
            }
        } else {
            (
                default_relay_okr_template(okr_id, &task),
                Some("(OKR: fallback template — provider unavailable)".to_string()),
            )
        };

        pending.okr = okr;
        pending.draft_note = draft_note;
        pending
    }

    /// Get the approval prompt text
    fn approval_prompt(&self) -> String {
        let krs: Vec<String> = self
            .okr
            .key_results
            .iter()
            .map(|kr| format!("  • {} (target: {} {})", kr.title, kr.target_value, kr.unit))
            .collect();

        let note_line = self
            .draft_note
            .as_deref()
            .map(|note| format!("{}\n", note))
            .unwrap_or_default();

        format!(
            "⚠️  Relay OKR Draft\n\n\
            Task: {task}\n\
            Agents: {agents} | Model: {model}\n\n\
            {note_line}\
            Objective: {objective}\n\n\
            Key Results:\n{key_results}\n\n\
            Press [A] to approve or [D] to deny",
            task = truncate_with_ellipsis(&self.task, 100),
            agents = self.agent_count,
            model = self.model,
            note_line = note_line,
            objective = self.okr.title,
            key_results = krs.join("\n"),
        )
    }
}


struct PlannedRelayProfile {
    #[serde(default)]
    name: String,
    #[serde(default)]
    specialty: String,
    #[serde(default)]
    mission: String,
    #[serde(default)]
    capabilities: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct PlannedRelayResponse {
    #[serde(default)]
    profiles: Vec<PlannedRelayProfile>,
}

#[derive(Debug, Clone, Deserialize)]
struct RelaySpawnDecision {
    #[serde(default)]
    spawn: bool,
    #[serde(default)]
    reason: String,
    #[serde(default)]
    profile: Option<PlannedRelayProfile>,
}

#[derive(Debug, Clone, Deserialize)]
struct PlannedOkrKeyResult {
    #[serde(default)]
    title: String,
    #[serde(default)]
    target_value: f64,
    #[serde(default = "default_okr_unit")]
    unit: String,
}

#[derive(Debug, Clone, Deserialize)]
struct PlannedOkrDraft {
    #[serde(default)]
    title: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    key_results: Vec<PlannedOkrKeyResult>,
}

fn default_okr_unit() -> String {
    "%".to_string()
}

fn slugify_label(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut last_dash = false;

    for ch in value.chars() {
        let ch = ch.to_ascii_lowercase();
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }

    out.trim_matches('-').to_string()
}

fn sanitize_relay_agent_name(value: &str) -> String {
    let raw = slugify_label(value);
    let base = if raw.is_empty() {
        "auto-specialist".to_string()
    } else if raw.starts_with("auto-") {
        raw
    } else {
        format!("auto-{raw}")
    };

    truncate_with_ellipsis(&base, 48)
        .trim_end_matches("...")
        .to_string()
}

fn sanitize_spawned_agent_name(value: &str) -> String {
    let slug = slugify_label(value);
    let bounded = truncate_with_ellipsis(&slug, 48);
    bounded.trim_end_matches("...").to_string()
}

fn unique_relay_agent_name(base: &str, existing: &[String]) -> String {
    if !existing.iter().any(|name| name == base) {
        return base.to_string();
    }

    let mut suffix = 2usize;
    loop {
        let candidate = format!("{base}-{suffix}");
        if !existing.iter().any(|name| name == &candidate) {
            return candidate;
        }
        suffix += 1;
    }
}

fn relay_instruction_from_plan(name: &str, specialty: &str, mission: &str) -> String {
    format!(
        "You are @{name}.\n\
         Specialty: {specialty}.\n\
         Mission: {mission}\n\n\
         This is a protocol-first relay conversation. Treat incoming handoffs as authoritative context.\n\
         Keep responses concise, concrete, and useful for the next specialist.\n\
         Include one clear recommendation for what the next agent should do.\n\
         If the task is too large for the current team, explicitly call out missing specialties and handoff boundaries.",
    )
}

fn build_runtime_profile_from_plan(
    profile: PlannedRelayProfile,
    existing: &[String],
) -> Option<(String, String, Vec<String>)> {
    let specialty = if profile.specialty.trim().is_empty() {
        "generalist".to_string()
    } else {
        profile.specialty.trim().to_string()
    };

    let mission = if profile.mission.trim().is_empty() {
        "Advance the relay with concrete next actions and clear handoffs.".to_string()
    } else {
        profile.mission.trim().to_string()
    };

    let base_name = if profile.name.trim().is_empty() {
        format!("auto-{}", slugify_label(&specialty))
    } else {
        profile.name.trim().to_string()
    };

    let sanitized = sanitize_relay_agent_name(&base_name);
    let name = unique_relay_agent_name(&sanitized, existing);
    if name.trim().is_empty() {
        return None;
    }

    let mut capabilities: Vec<String> = Vec::new();
    let specialty_cap = slugify_label(&specialty);
    if !specialty_cap.is_empty() {
        capabilities.push(specialty_cap);
    }

    for capability in profile.capabilities {
        let normalized = slugify_label(&capability);
        if !normalized.is_empty() && !capabilities.contains(&normalized) {
            capabilities.push(normalized);
        }
    }

    crate::autochat::ensure_required_relay_capabilities(&mut capabilities);

    let instructions = relay_instruction_from_plan(&name, &specialty, &mission);
    Some((name, instructions, capabilities))
}

fn extract_json_payload<T: DeserializeOwned>(text: &str) -> Option<T> {
    let trimmed = text.trim();
    if let Ok(value) = serde_json::from_str::<T>(trimmed) {
        return Some(value);
    }

    if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}'))
        && start < end
        && let Ok(value) = serde_json::from_str::<T>(&trimmed[start..=end])
    {
        return Some(value);
    }

    if let (Some(start), Some(end)) = (trimmed.find('['), trimmed.rfind(']'))
        && start < end
        && let Ok(value) = serde_json::from_str::<T>(&trimmed[start..=end])
    {
        return Some(value);
    }

    None
}

fn default_relay_okr_template(okr_id: Uuid, task: &str) -> Okr {
    let mut okr = Okr::new(
        format!("Relay: {}", truncate_with_ellipsis(task, 60)),
        format!("Execute relay task: {}", task),
    );
    okr.id = okr_id;

    okr.add_key_result(KeyResult::new(
        okr_id,
        "Relay completes all rounds",
        100.0,
        "%",
    ));
    okr.add_key_result(KeyResult::new(
        okr_id,
        "Team produces actionable handoff",
        1.0,
        "count",
    ));
    okr.add_key_result(KeyResult::new(okr_id, "No critical errors", 0.0, "count"));

    okr
}

fn okr_from_planned_draft(okr_id: Uuid, task: &str, planned: PlannedOkrDraft) -> Okr {
    let title = if planned.title.trim().is_empty() {
        format!("Relay: {}", truncate_with_ellipsis(task, 60))
    } else {
        planned.title.trim().to_string()
    };

    let description = if planned.description.trim().is_empty() {
        format!("Execute relay task: {}", task)
    } else {
        planned.description.trim().to_string()
    };

    let mut okr = Okr::new(title, description);
    okr.id = okr_id;

    for kr in planned.key_results.into_iter().take(7) {
        if kr.title.trim().is_empty() {
            continue;
        }

        let unit = if kr.unit.trim().is_empty() {
            default_okr_unit()
        } else {
            kr.unit
        };

        okr.add_key_result(KeyResult::new(
            okr_id,
            kr.title.trim().to_string(),
            kr.target_value.max(0.0),
            unit,
        ));
    }

    if okr.key_results.is_empty() {
        default_relay_okr_template(okr_id, task)
    } else {
        okr
    }
}

async fn plan_okr_draft_with_registry(
