use super::{AccessMode, ApprovalPolicy, PermissionProfile, PolicyRequirements, SandboxMode};

fn pick<T: Copy + Eq>(allowed: &[T], candidate: T) -> T {
    if allowed.is_empty() || allowed.contains(&candidate) {
        candidate
    } else {
        allowed[0]
    }
}

impl PolicyRequirements {
    pub fn access_mode(&self, candidate: AccessMode) -> AccessMode {
        pick(&self.allowed_access_modes, candidate)
    }

    pub fn sandbox_mode(&self, candidate: SandboxMode) -> SandboxMode {
        pick(&self.allowed_sandbox_modes, candidate)
    }

    pub fn approval_policy(&self, candidate: ApprovalPolicy) -> ApprovalPolicy {
        pick(&self.allowed_approval_policies, candidate)
    }

    pub fn permission_profile(&self, candidate: PermissionProfile) -> PermissionProfile {
        pick(&self.allowed_permission_profiles, candidate)
    }
}
