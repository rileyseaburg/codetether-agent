//! Application callback bindings for classified leading commands.
mod attachments;
mod diagnostics;
mod execution;
mod settings;

/// Declare leading command routing while retaining application callbacks.
///
/// Groups select attachment actions, diagnostics, execution and preference
/// commands. The supplied dispatcher retains app, cwd, session and registry.
/// # Examples
/// ```text
/// codetether_slash_command::declare_leading_commands!(
///     Dispatch<'_>, Presentation, Setting; image, file, editor;
///     context, mcp; go, ask; settings,
/// );
/// ```
#[macro_export]
macro_rules! declare_leading_commands {
    ($dispatch:ty, $presentation:path, $kind:path;
     $image:path, $file:path, $edit:path;
     $context:path, $mcp:path; $go:path, $ask:path; $settings:path $(,)?) => {
        mod leading {
            use $crate::routing::LeadingCommand;
            $crate::__leading_attachments!($dispatch, $presentation, $image, $file, $edit);
            $crate::__leading_diagnostics!($dispatch, $context, $mcp);
            $crate::__leading_execution!($dispatch, $go, $ask);
            $crate::__leading_settings!($dispatch, $kind, $settings);
            pub(super) async fn run(host: &mut $dispatch, kind: LeadingCommand, rest: &str) {
                match kind {
                    LeadingCommand::Image | LeadingCommand::File | LeadingCommand::Edit => {
                        attachments::run(host, kind, rest)
                    }
                    LeadingCommand::Go | LeadingCommand::Ask => {
                        execution::run(host, kind, rest).await
                    }
                    LeadingCommand::AutoApply
                    | LeadingCommand::Network
                    | LeadingCommand::Autocomplete => settings::run(host, kind, rest).await,
                    LeadingCommand::Context | LeadingCommand::Mcp => {
                        diagnostics::run(host, kind, rest).await
                    }
                }
            }
        }
    };
}
