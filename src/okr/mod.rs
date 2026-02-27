//! OKR (Objectives and Key Results) domain models
//!
//! This module provides first-class OKR entities for operational control:
//! - `Okr` - An objective with measurable key results
//! - `KeyResult` - Quantifiable outcomes tied to objectives
//! - `OkrRun` - An execution instance of an OKR
//! - `ApprovalDecision` - Approve/deny gate decisions
//! - `KrOutcome` - Evidence-backed outcomes for key results
//! - `OkrRepository` - File-based persistence with CRUD operations

pub mod persistence;

pub use persistence::OkrRepository;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A high-level objective with associated key results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Okr {
    /// Unique identifier for this OKR
    pub id: Uuid,

    /// Human-readable title of the objective
    pub title: String,

    /// Detailed description of what this objective aims to achieve
    pub description: String,

    /// Current status of the OKR
    #[serde(default)]
    pub status: OkrStatus,

    /// Key results that measure success
    #[serde(default)]
    pub key_results: Vec<KeyResult>,

    /// Owner of this OKR (user ID or team)
    #[serde(default)]
    pub owner: Option<String>,

    /// Tenant ID for multi-tenant isolation
    #[serde(default)]
    pub tenant_id: Option<String>,

    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,

    /// Creation timestamp
    #[serde(default = "utc_now")]
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    #[serde(default = "utc_now")]
    pub updated_at: DateTime<Utc>,

    /// Target completion date
    #[serde(default)]
    pub target_date: Option<DateTime<Utc>>,
}

impl Okr {
    /// Create a new OKR with a generated UUID
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            description: description.into(),
            status: OkrStatus::Draft,
            key_results: Vec::new(),
            owner: None,
            tenant_id: None,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            target_date: None,
        }
    }

    /// Validate the OKR structure
    pub fn validate(&self) -> Result<(), OkrValidationError> {
        if self.title.trim().is_empty() {
            return Err(OkrValidationError::EmptyTitle);
        }
        if self.key_results.is_empty() {
            return Err(OkrValidationError::NoKeyResults);
        }
        for kr in &self.key_results {
            kr.validate()?;
        }
        Ok(())
    }

    /// Calculate overall progress (0.0 to 1.0) across all key results
    pub fn progress(&self) -> f64 {
        if self.key_results.is_empty() {
            return 0.0;
        }
        let total: f64 = self.key_results.iter().map(|kr| kr.progress()).sum();
        total / self.key_results.len() as f64
    }

    /// Check if all key results are complete
    pub fn is_complete(&self) -> bool {
        self.key_results.iter().all(|kr| kr.is_complete())
    }

    /// Add a key result to this OKR
    pub fn add_key_result(&mut self, kr: KeyResult) {
        self.key_results.push(kr);
        self.updated_at = Utc::now();
    }
}

/// OKR status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum OkrStatus {
    /// Draft - not yet approved
    #[default]
    Draft,

    /// Active - approved and in progress
    Active,

    /// Completed - all key results achieved
    Completed,

    /// Cancelled - abandoned before completion
    Cancelled,

    /// OnHold - temporarily paused
    OnHold,
}

/// A measurable key result within an OKR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyResult {
    /// Unique identifier for this key result
    pub id: Uuid,

    /// Parent OKR ID
    pub okr_id: Uuid,

    /// Human-readable title
    pub title: String,

    /// Detailed description
    pub description: String,

    /// Target value (numeric for percentage/count metrics)
    pub target_value: f64,

    /// Current value (progress)
    #[serde(default)]
    pub current_value: f64,

    /// Unit of measurement (e.g., "%", "count", "files", "tests")
    #[serde(default = "default_unit")]
    pub unit: String,

    /// Type of metric
    #[serde(default)]
    pub metric_type: KrMetricType,

    /// Current status
    #[serde(default)]
    pub status: KeyResultStatus,

    /// Evidence/outcomes linked to this KR
    #[serde(default)]
    pub outcomes: Vec<KrOutcome>,

    /// Creation timestamp
    #[serde(default = "utc_now")]
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    #[serde(default = "utc_now")]
    pub updated_at: DateTime<Utc>,
}

impl KeyResult {
    /// Create a new key result
    pub fn new(
        okr_id: Uuid,
        title: impl Into<String>,
        target_value: f64,
        unit: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            okr_id,
            title: title.into(),
            description: String::new(),
            target_value,
            current_value: 0.0,
            unit: unit.into(),
            metric_type: KrMetricType::Progress,
            status: KeyResultStatus::Pending,
            outcomes: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Validate the key result
    pub fn validate(&self) -> Result<(), OkrValidationError> {
        if self.title.trim().is_empty() {
            return Err(OkrValidationError::EmptyKeyResultTitle);
        }
        if self.target_value < 0.0 {
            return Err(OkrValidationError::InvalidTargetValue);
        }
        Ok(())
    }

    /// Calculate progress as a ratio (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.target_value == 0.0 {
            return 0.0;
        }
        (self.current_value / self.target_value).clamp(0.0, 1.0)
    }

    /// Check if the key result is complete
    pub fn is_complete(&self) -> bool {
        self.status == KeyResultStatus::Completed || self.current_value >= self.target_value
    }

    /// Add an outcome to this key result
    pub fn add_outcome(&mut self, outcome: KrOutcome) {
        self.outcomes.push(outcome);
        self.updated_at = Utc::now();
    }

    /// Update current value and recalculate status
    pub fn update_progress(&mut self, value: f64) {
        self.current_value = value;
        self.updated_at = Utc::now();
        if self.is_complete() {
            self.status = KeyResultStatus::Completed;
        } else if self.current_value > 0.0 {
            self.status = KeyResultStatus::InProgress;
        }
    }
}

/// Type of metric for a key result
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum KrMetricType {
    /// Progress percentage (0-100)
    #[default]
    Progress,

    /// Count of items
    Count,

    /// Boolean completion
    Binary,

    /// Latency/duration (lower is better)
    Latency,

    /// Quality score (0-1 or 0-100)
    Quality,
}

/// Status of a key result
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum KeyResultStatus {
    /// Not yet started
    #[default]
    Pending,

    /// Actively being worked on
    InProgress,

    /// Achieved the target
    Completed,

    /// At risk of missing target
    AtRisk,

    /// Failed to achieve target
    Failed,
}

/// An execution run of an OKR (multiple runs per OKR are allowed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkrRun {
    /// Unique identifier for this run
    pub id: Uuid,

    /// Parent OKR ID
    pub okr_id: Uuid,

    /// Human-readable name for this run
    pub name: String,

    /// Current status of the run
    #[serde(default)]
    pub status: OkrRunStatus,

    /// Correlation ID linking to relay/session
    #[serde(default)]
    pub correlation_id: Option<String>,

    /// Relay checkpoint ID for resume capability
    #[serde(default)]
    pub relay_checkpoint_id: Option<String>,

    /// Session ID if applicable
    #[serde(default)]
    pub session_id: Option<String>,

    /// Progress per key result (kr_id -> progress)
    #[serde(default)]
    pub kr_progress: std::collections::HashMap<String, f64>,

    /// Approval decision for this run
    #[serde(default)]
    pub approval: Option<ApprovalDecision>,

    /// List of outcomes achieved in this run
    #[serde(default)]
    pub outcomes: Vec<KrOutcome>,

    /// Iteration count for this run
    #[serde(default)]
    pub iterations: u32,

    /// Started timestamp
    #[serde(default = "utc_now")]
    pub started_at: DateTime<Utc>,

    /// Completed timestamp (if finished)
    #[serde(default)]
    pub completed_at: Option<DateTime<Utc>>,

    /// Last update timestamp
    #[serde(default = "utc_now")]
    pub updated_at: DateTime<Utc>,
}

impl OkrRun {
    /// Create a new OKR run
    pub fn new(okr_id: Uuid, name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            okr_id,
            name: name.into(),
            status: OkrRunStatus::Draft,
            correlation_id: None,
            relay_checkpoint_id: None,
            session_id: None,
            kr_progress: std::collections::HashMap::new(),
            approval: None,
            outcomes: Vec::new(),
            iterations: 0,
            started_at: now,
            completed_at: None,
            updated_at: now,
        }
    }

    /// Validate the run
    pub fn validate(&self) -> Result<(), OkrValidationError> {
        if self.name.trim().is_empty() {
            return Err(OkrValidationError::EmptyRunName);
        }
        Ok(())
    }

    /// Submit for approval
    pub fn submit_for_approval(&mut self) -> Result<(), OkrValidationError> {
        if self.status != OkrRunStatus::Draft {
            return Err(OkrValidationError::InvalidStatusTransition);
        }
        self.status = OkrRunStatus::PendingApproval;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Record an approval decision
    pub fn record_decision(&mut self, decision: ApprovalDecision) {
        self.approval = Some(decision.clone());
        self.updated_at = Utc::now();
        match decision.decision {
            ApprovalChoice::Approved => {
                self.status = OkrRunStatus::Approved;
            }
            ApprovalChoice::Denied => {
                self.status = OkrRunStatus::Denied;
            }
        }
    }

    /// Start execution
    pub fn start(&mut self) -> Result<(), OkrValidationError> {
        if self.status != OkrRunStatus::Approved {
            return Err(OkrValidationError::NotApproved);
        }
        self.status = OkrRunStatus::Running;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Mark as complete
    pub fn complete(&mut self) {
        self.status = OkrRunStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Update key result progress
    pub fn update_kr_progress(&mut self, kr_id: &str, progress: f64) {
        self.kr_progress.insert(kr_id.to_string(), progress);
        self.updated_at = Utc::now();
    }

    /// Check if run can be resumed
    pub fn is_resumable(&self) -> bool {
        matches!(
            self.status,
            OkrRunStatus::Running | OkrRunStatus::Paused | OkrRunStatus::WaitingApproval
        )
    }
}

/// Status of an OKR run
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum OkrRunStatus {
    /// Draft - not yet submitted
    #[default]
    Draft,

    /// Pending approval
    PendingApproval,

    /// Approved to run
    Approved,

    /// Actively running
    Running,

    /// Paused (can resume)
    Paused,

    /// Waiting for something
    WaitingApproval,

    /// Successfully completed
    Completed,

    /// Failed to complete
    Failed,

    /// Denied approval
    Denied,

    /// Cancelled
    Cancelled,
}

/// Approval decision for an OKR run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalDecision {
    /// Unique identifier for this decision
    pub id: Uuid,

    /// ID of the run this decision applies to
    pub run_id: Uuid,

    /// The actual decision
    pub decision: ApprovalChoice,

    /// Reason for the decision
    #[serde(default)]
    pub reason: String,

    /// Who made the decision
    #[serde(default)]
    pub approver: Option<String>,

    /// Additional context/metadata
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,

    /// Timestamp of the decision
    #[serde(default = "utc_now")]
    pub decided_at: DateTime<Utc>,
}

impl ApprovalDecision {
    /// Create an approval
    pub fn approve(run_id: Uuid, reason: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            run_id,
            decision: ApprovalChoice::Approved,
            reason: reason.into(),
            approver: None,
            metadata: std::collections::HashMap::new(),
            decided_at: Utc::now(),
        }
    }

    /// Create a denial
    pub fn deny(run_id: Uuid, reason: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            run_id,
            decision: ApprovalChoice::Denied,
            reason: reason.into(),
            approver: None,
            metadata: std::collections::HashMap::new(),
            decided_at: Utc::now(),
        }
    }
}

/// Approval choice enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalChoice {
    /// Approved to proceed
    Approved,

    /// Denied
    Denied,
}

/// Evidence-backed outcome for a key result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrOutcome {
    /// Unique identifier for this outcome
    pub id: Uuid,

    /// Key result this outcome belongs to
    pub kr_id: Uuid,

    /// OKR run this outcome is part of
    #[serde(default)]
    pub run_id: Option<Uuid>,

    /// Description of the outcome/evidence
    pub description: String,

    /// Type of outcome
    #[serde(default)]
    pub outcome_type: KrOutcomeType,

    /// Numeric value contributed (if applicable)
    #[serde(default)]
    pub value: Option<f64>,

    /// Evidence links (URLs, file paths, etc.)
    #[serde(default)]
    pub evidence: Vec<String>,

    /// Who/what generated this outcome
    #[serde(default)]
    pub source: String,

    /// Timestamp
    #[serde(default = "utc_now")]
    pub created_at: DateTime<Utc>,
}

impl KrOutcome {
    /// Create a new outcome
    pub fn new(kr_id: Uuid, description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            kr_id,
            run_id: None,
            description: description.into(),
            outcome_type: KrOutcomeType::Evidence,
            value: None,
            evidence: Vec::new(),
            source: String::new(),
            created_at: Utc::now(),
        }
    }

    /// Create a metric outcome with a value
    pub fn with_value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Add evidence link
    pub fn add_evidence(mut self, evidence: impl Into<String>) -> Self {
        self.evidence.push(evidence.into());
        self
    }
}

/// Type of key result outcome
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum KrOutcomeType {
    /// Evidence/documentation
    #[default]
    Evidence,

    /// Test pass
    TestPass,

    /// Code change
    CodeChange,

    /// Bug fix
    BugFix,

    /// Feature delivered
    FeatureDelivered,

    /// Metric achievement
    MetricAchievement,

    /// Review passed
    ReviewPassed,

    /// Deployment
    Deployment,
}

/// Validation errors for OKR entities
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OkrValidationError {
    /// Objective title is empty
    EmptyTitle,

    /// No key results defined
    NoKeyResults,

    /// Key result title is empty
    EmptyKeyResultTitle,

    /// Target value is invalid
    InvalidTargetValue,

    /// Run name is empty
    EmptyRunName,

    /// Invalid status transition
    InvalidStatusTransition,

    /// Run not approved
    NotApproved,
}

impl std::fmt::Display for OkrValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OkrValidationError::EmptyTitle => write!(f, "objective title cannot be empty"),
            OkrValidationError::NoKeyResults => write!(f, "at least one key result is required"),
            OkrValidationError::EmptyKeyResultTitle => {
                write!(f, "key result title cannot be empty")
            }
            OkrValidationError::InvalidTargetValue => {
                write!(f, "target value must be non-negative")
            }
            OkrValidationError::EmptyRunName => write!(f, "run name cannot be empty"),
            OkrValidationError::InvalidStatusTransition => {
                write!(f, "invalid status transition")
            }
            OkrValidationError::NotApproved => write!(f, "run must be approved before starting"),
        }
    }
}

impl std::error::Error for OkrValidationError {}

/// Helper to get current UTC time
fn utc_now() -> DateTime<Utc> {
    Utc::now()
}

/// Default unit value
fn default_unit() -> String {
    "%".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_okr_creation() {
        let okr = Okr::new("Test Objective", "Description");
        assert_eq!(okr.title, "Test Objective");
        assert_eq!(okr.status, OkrStatus::Draft);
        assert!(okr.validate().is_err()); // No key results
    }

    #[test]
    fn test_okr_with_key_results() {
        let mut okr = Okr::new("Test Objective", "Description");
        let kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
        okr.add_key_result(kr);
        assert!(okr.validate().is_ok());
    }

    #[test]
    fn test_key_result_progress() {
        let kr = KeyResult::new(Uuid::new_v4(), "Test KR", 100.0, "%");
        assert_eq!(kr.progress(), 0.0);

        let mut kr = kr;
        kr.update_progress(50.0);
        assert!((kr.progress() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_okr_run_workflow() {
        let okr_id = Uuid::new_v4();
        let mut run = OkrRun::new(okr_id, "Q1 2024 Run");

        // Submit for approval
        run.submit_for_approval().unwrap();
        assert_eq!(run.status, OkrRunStatus::PendingApproval);

        // Record approval
        run.record_decision(ApprovalDecision::approve(run.id, "Looks good"));
        assert_eq!(run.status, OkrRunStatus::Approved);

        // Start execution
        run.start().unwrap();
        assert_eq!(run.status, OkrRunStatus::Running);

        // Update progress
        run.update_kr_progress("kr-1", 0.5);

        // Complete
        run.complete();
        assert_eq!(run.status, OkrRunStatus::Completed);
    }

    #[test]
    fn test_outcome_creation() {
        let outcome = KrOutcome::new(Uuid::new_v4(), "Fixed bug in auth")
            .with_value(1.0)
            .add_evidence("commit:abc123");

        assert_eq!(outcome.value, Some(1.0));
        assert!(outcome.evidence.contains(&"commit:abc123".to_string()));
    }
}
