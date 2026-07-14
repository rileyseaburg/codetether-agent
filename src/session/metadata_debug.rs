//! Redacted debug formatting for session metadata.

use super::super::SessionMetadata;

impl std::fmt::Debug for SessionMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionMetadata")
            .field("directory", &self.directory)
            .field("model", &self.model)
            .field("knowledge_snapshot", &self.knowledge_snapshot)
            .field("provenance", &self.provenance)
            .field("auto_apply_edits", &self.auto_apply_edits)
            .field("allow_network", &self.allow_network)
            .field("prior_context_allowed", &self.prior_context_allowed)
            .field(
                "prior_context_turn_allowed",
                &self.prior_context_turn_allowed,
            )
            .field(
                "inherited_prior_context_allowed",
                &self.inherited_prior_context_allowed,
            )
            .field("slash_autocomplete", &self.slash_autocomplete)
            .field("use_worktree", &self.use_worktree)
            .field("shared", &self.shared)
            .field("share_url", &self.share_url)
            .field("rlm", &self.rlm)
            .field("context_policy", &self.context_policy)
            .field("delegation", &self.delegation)
            .field("run_checkpoint", &self.run_checkpoint)
            .field("history_sink", &super::sink::history_sink(self))
            .field(
                "provider_keys",
                &self.provider_keys.as_ref().map(|_| "<redacted>"),
            )
            .field(
                "subcall_provider",
                &self.subcall_provider.as_ref().map(|_| "<provider>"),
            )
            .field("subcall_model_name", &self.subcall_model_name)
            .finish()
    }
}
