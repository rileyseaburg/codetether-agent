//! OKR Tool - Manage Objectives and Key Results for goal tracking.
//!
//! Exposes full CRUD operations on OKRs and OKR Runs so that LLMs can
//! create, query, update, and delete objectives and their execution runs.

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::OnceCell;
use uuid::Uuid;

use crate::okr::{KeyResult, Okr, OkrRepository, OkrRun, OkrRunStatus, OkrStatus};

/// Lazily-initialized shared OKR repository
static OKR_REPO: OnceCell<Arc<OkrRepository>> = OnceCell::const_new();

async fn get_repo() -> Result<&'static Arc<OkrRepository>> {
    OKR_REPO
        .get_or_try_init(|| async {
            let repo = OkrRepository::from_config().await?;
            Ok::<_, anyhow::Error>(Arc::new(repo))
        })
        .await
}

pub struct OkrTool;

impl Default for OkrTool {
    fn default() -> Self {
        Self::new()
    }
}

impl OkrTool {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Deserialize)]
struct Params {
    action: String,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    owner: Option<String>,
    #[serde(default)]
    tenant_id: Option<String>,
    #[serde(default)]
    key_results: Option<Vec<KrParam>>,
    // Run-specific fields
    #[serde(default)]
    okr_id: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    correlation_id: Option<String>,
    #[serde(default)]
    session_id: Option<String>,
    #[serde(default)]
    checkpoint_id: Option<String>,
}

#[derive(Deserialize)]
struct KrParam {
    title: String,
    target_value: f64,
    #[serde(default = "default_unit")]
    unit: String,
}

fn default_unit() -> String {
    "%".to_string()
}

fn parse_uuid(s: &str, field: &str) -> Result<Uuid> {
    Uuid::parse_str(s).with_context(|| format!("Invalid UUID for {field}: {s}"))
}

fn parse_okr_status(s: &str) -> Result<OkrStatus> {
    match s {
        "draft" => Ok(OkrStatus::Draft),
        "active" => Ok(OkrStatus::Active),
        "completed" => Ok(OkrStatus::Completed),
        "cancelled" => Ok(OkrStatus::Cancelled),
        "on_hold" => Ok(OkrStatus::OnHold),
        _ => anyhow::bail!(
            "Unknown OKR status: {s}. Use: draft, active, completed, cancelled, on_hold"
        ),
    }
}

fn parse_run_status(s: &str) -> Result<OkrRunStatus> {
    match s {
        "draft" => Ok(OkrRunStatus::Draft),
        "pending_approval" => Ok(OkrRunStatus::PendingApproval),
        "approved" => Ok(OkrRunStatus::Approved),
        "running" => Ok(OkrRunStatus::Running),
        "paused" => Ok(OkrRunStatus::Paused),
        "waiting_approval" => Ok(OkrRunStatus::WaitingApproval),
        "completed" => Ok(OkrRunStatus::Completed),
        "failed" => Ok(OkrRunStatus::Failed),
        "denied" => Ok(OkrRunStatus::Denied),
        "cancelled" => Ok(OkrRunStatus::Cancelled),
        _ => anyhow::bail!("Unknown run status: {s}"),
    }
}

#[async_trait]
impl Tool for OkrTool {
    fn id(&self) -> &str {
        "okr"
    }
    fn name(&self) -> &str {
        "OKR Manager"
    }
    fn description(&self) -> &str {
        "Manage Objectives and Key Results (OKRs) and their execution runs. \
         Actions: create_okr, get_okr, update_okr, delete_okr, list_okrs, query_okrs, \
         create_run, get_run, update_run, delete_run, list_runs, query_runs, stats."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": [
                        "create_okr", "get_okr", "update_okr", "delete_okr",
                        "list_okrs", "query_okrs",
                        "create_run", "get_run", "update_run", "delete_run",
                        "list_runs", "query_runs",
                        "stats"
                    ],
                    "description": "Action to perform"
                },
                "id": {"type": "string", "description": "UUID of the OKR or run"},
                "title": {"type": "string", "description": "OKR title (for create/update)"},
                "description": {"type": "string", "description": "OKR description"},
                "status": {
                    "type": "string",
                    "description": "Status filter or new status. OKR: draft/active/completed/cancelled/on_hold. Run: draft/pending_approval/approved/running/paused/waiting_approval/completed/failed/denied/cancelled"
                },
                "owner": {"type": "string", "description": "Owner filter or assignment"},
                "tenant_id": {"type": "string", "description": "Tenant ID filter or assignment"},
                "key_results": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "title": {"type": "string"},
                            "target_value": {"type": "number"},
                            "unit": {"type": "string", "default": "%"}
                        },
                        "required": ["title", "target_value"]
                    },
                    "description": "Key results to add (for create_okr)"
                },
                "okr_id": {"type": "string", "description": "Parent OKR UUID (for run operations)"},
                "name": {"type": "string", "description": "Run name (for create_run)"},
                "correlation_id": {"type": "string", "description": "Correlation ID for run queries"},
                "session_id": {"type": "string", "description": "Session ID for run queries"},
                "checkpoint_id": {"type": "string", "description": "Relay checkpoint ID for run queries"}
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid OKR tool params")?;
        let repo = get_repo().await?;

        match p.action.as_str() {
            // ===== OKR CRUD =====
            "create_okr" => {
                let title = p.title.ok_or_else(|| anyhow::anyhow!("title required"))?;
                let desc = p.description.unwrap_or_default();
                let mut okr = Okr::new(&title, &desc);

                if let Some(owner) = p.owner {
                    okr.owner = Some(owner);
                }
                if let Some(tid) = p.tenant_id {
                    okr.tenant_id = Some(tid);
                }

                let krs = p
                    .key_results
                    .ok_or_else(|| anyhow::anyhow!("key_results required (at least one)"))?;
                for kr_p in krs {
                    let kr = KeyResult::new(okr.id, &kr_p.title, kr_p.target_value, &kr_p.unit);
                    okr.add_key_result(kr);
                }

                let created = repo.create_okr(okr).await?;
                Ok(ToolResult::success(serde_json::to_string_pretty(&created)?)
                    .with_metadata("okr_id", json!(created.id.to_string())))
            }

            "get_okr" => {
                let id = parse_uuid(
                    p.id.as_deref()
                        .ok_or_else(|| anyhow::anyhow!("id required"))?,
                    "id",
                )?;
                match repo.get_okr(id).await? {
                    Some(okr) => Ok(ToolResult::success(serde_json::to_string_pretty(&okr)?)),
                    None => Ok(ToolResult::error(format!("OKR not found: {id}"))),
                }
            }

            "update_okr" => {
                let id = parse_uuid(
                    p.id.as_deref()
                        .ok_or_else(|| anyhow::anyhow!("id required"))?,
                    "id",
                )?;
                let mut okr = repo
                    .get_okr(id)
                    .await?
                    .ok_or_else(|| anyhow::anyhow!("OKR not found: {id}"))?;

                if let Some(title) = p.title {
                    okr.title = title;
                }
                if let Some(desc) = p.description {
                    okr.description = desc;
                }
                if let Some(status_str) = p.status {
                    okr.status = parse_okr_status(&status_str)?;
                }
                if let Some(owner) = p.owner {
                    okr.owner = Some(owner);
                }
                if let Some(tid) = p.tenant_id {
                    okr.tenant_id = Some(tid);
                }

                let updated = repo.update_okr(okr).await?;
                Ok(ToolResult::success(serde_json::to_string_pretty(&updated)?))
            }

            "delete_okr" => {
                let id = parse_uuid(
                    p.id.as_deref()
                        .ok_or_else(|| anyhow::anyhow!("id required"))?,
                    "id",
                )?;
                let deleted = repo.delete_okr(id).await?;
                if deleted {
                    Ok(ToolResult::success(format!("Deleted OKR: {id}")))
                } else {
                    Ok(ToolResult::error(format!("OKR not found: {id}")))
                }
            }

            "list_okrs" => {
                let okrs = repo.list_okrs().await?;
                if okrs.is_empty() {
                    return Ok(ToolResult::success("No OKRs found"));
                }
                Ok(ToolResult::success(serde_json::to_string_pretty(&okrs)?)
                    .with_metadata("count", json!(okrs.len())))
            }

            "query_okrs" => {
                let okrs = if let Some(status_str) = p.status {
                    let status = parse_okr_status(&status_str)?;
                    repo.query_okrs_by_status(status).await?
                } else if let Some(owner) = p.owner {
                    repo.query_okrs_by_owner(&owner).await?
                } else if let Some(tid) = p.tenant_id {
                    repo.query_okrs_by_tenant(&tid).await?
                } else {
                    anyhow::bail!("query_okrs requires status, owner, or tenant_id");
                };
                Ok(ToolResult::success(serde_json::to_string_pretty(&okrs)?)
                    .with_metadata("count", json!(okrs.len())))
            }

            // ===== Run CRUD =====
            "create_run" => {
                let okr_id = parse_uuid(
                    p.okr_id
                        .as_deref()
                        .ok_or_else(|| anyhow::anyhow!("okr_id required"))?,
                    "okr_id",
                )?;
                let name = p.name.ok_or_else(|| anyhow::anyhow!("name required"))?;
                let mut run = OkrRun::new(okr_id, &name);

                if let Some(cid) = p.correlation_id {
                    run.correlation_id = Some(cid);
                }
                if let Some(sid) = p.session_id {
                    run.session_id = Some(sid);
                }

                let created = repo.create_run(run).await?;
                Ok(ToolResult::success(serde_json::to_string_pretty(&created)?)
                    .with_metadata("run_id", json!(created.id.to_string())))
            }

            "get_run" => {
                let id = parse_uuid(
                    p.id.as_deref()
                        .ok_or_else(|| anyhow::anyhow!("id required"))?,
                    "id",
                )?;
                match repo.get_run(id).await? {
                    Some(run) => Ok(ToolResult::success(serde_json::to_string_pretty(&run)?)),
                    None => Ok(ToolResult::error(format!("Run not found: {id}"))),
                }
            }

            "update_run" => {
                let id = parse_uuid(
                    p.id.as_deref()
                        .ok_or_else(|| anyhow::anyhow!("id required"))?,
                    "id",
                )?;
                let mut run = repo
                    .get_run(id)
                    .await?
                    .ok_or_else(|| anyhow::anyhow!("Run not found: {id}"))?;

                if let Some(name) = p.name {
                    run.name = name;
                }
                if let Some(status_str) = p.status {
                    run.status = parse_run_status(&status_str)?;
                }
                if let Some(cid) = p.correlation_id {
                    run.correlation_id = Some(cid);
                }
                if let Some(sid) = p.session_id {
                    run.session_id = Some(sid);
                }

                let updated = repo.update_run(run).await?;
                Ok(ToolResult::success(serde_json::to_string_pretty(&updated)?))
            }

            "delete_run" => {
                let id = parse_uuid(
                    p.id.as_deref()
                        .ok_or_else(|| anyhow::anyhow!("id required"))?,
                    "id",
                )?;
                let deleted = repo.delete_run(id).await?;
                if deleted {
                    Ok(ToolResult::success(format!("Deleted run: {id}")))
                } else {
                    Ok(ToolResult::error(format!("Run not found: {id}")))
                }
            }

            "list_runs" => {
                let runs = repo.list_runs().await?;
                if runs.is_empty() {
                    return Ok(ToolResult::success("No runs found"));
                }
                Ok(ToolResult::success(serde_json::to_string_pretty(&runs)?)
                    .with_metadata("count", json!(runs.len())))
            }

            "query_runs" => {
                let runs = if let Some(okr_id_str) = p.okr_id {
                    let okr_id = parse_uuid(&okr_id_str, "okr_id")?;
                    repo.query_runs_by_okr(okr_id).await?
                } else if let Some(status_str) = p.status {
                    let status = parse_run_status(&status_str)?;
                    repo.query_runs_by_status(status).await?
                } else if let Some(cid) = p.correlation_id {
                    repo.query_runs_by_correlation(&cid).await?
                } else if let Some(cpid) = p.checkpoint_id {
                    repo.query_runs_by_checkpoint(&cpid).await?
                } else if let Some(sid) = p.session_id {
                    repo.query_runs_by_session(&sid).await?
                } else {
                    anyhow::bail!(
                        "query_runs requires okr_id, status, correlation_id, checkpoint_id, or session_id"
                    );
                };
                Ok(ToolResult::success(serde_json::to_string_pretty(&runs)?)
                    .with_metadata("count", json!(runs.len())))
            }

            // ===== Stats =====
            "stats" => {
                let stats = repo.stats().await?;
                Ok(ToolResult::success(serde_json::to_string_pretty(&stats)?))
            }

            _ => Ok(ToolResult::error(format!(
                "Unknown action: {}. Use: create_okr, get_okr, update_okr, delete_okr, \
                 list_okrs, query_okrs, create_run, get_run, update_run, delete_run, \
                 list_runs, query_runs, stats",
                p.action
            ))),
        }
    }
}
