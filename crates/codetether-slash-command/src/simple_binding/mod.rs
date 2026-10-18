//! Application bindings for classified simple slash commands.
mod inspection;
mod memory;
mod panels;
mod sessions;
mod views;
/// Declare simple-command routing with explicit frontend callbacks.
///
/// State changes retain the selected view/layout enums and existing session,
/// inspector, monitor, notice and recall services.
/// # Examples
/// See the frontend dispatch module for the selected callbacks.
#[macro_export]
macro_rules! declare_simple_commands {
    ($dispatch:ty, $mode:path, $layout:path;
     $refresh:path, $import:path, $model:path, $new:path;
     $swarm:path, $chat:path; $audit:path, $git:path, $notice:path; $recall:path $(,)?) => {
        mod simple {
            use $crate::pipeline::SimpleCommand;
            $crate::__simple_panels!($dispatch, $mode);
            $crate::__simple_sessions!($dispatch, $mode, $refresh, $import, $model, $new);
            $crate::__simple_views!($dispatch, $mode, $layout, $swarm, $chat);
            $crate::__simple_inspection!($dispatch, $mode, $audit, $git, $notice);
            $crate::__simple_memory!($dispatch, $recall);
            pub(super) async fn run(host: &mut $dispatch, kind: SimpleCommand) {
                match kind {
                    SimpleCommand::Sessions
                    | SimpleCommand::ImportCodex
                    | SimpleCommand::New
                    | SimpleCommand::Model => sessions::run(host, kind).await,
                    SimpleCommand::Bus
                    | SimpleCommand::Lsp
                    | SimpleCommand::Rlm
                    | SimpleCommand::Latency
                    | SimpleCommand::Transport
                    | SimpleCommand::Inspector
                    | SimpleCommand::AuditLoop
                    | SimpleCommand::Settings
                    | SimpleCommand::Subagents => panels::run(host, kind),
                    SimpleCommand::Help
                    | SimpleCommand::Swarm
                    | SimpleCommand::Ralph
                    | SimpleCommand::Chat
                    | SimpleCommand::Webview
                    | SimpleCommand::Classic
                    | SimpleCommand::Symbols => views::run(host, kind),
                    SimpleCommand::Audit
                    | SimpleCommand::Git
                    | SimpleCommand::Plugin
                    | SimpleCommand::Keys => inspection::run(host, kind).await,
                    SimpleCommand::Recall => memory::run(host, kind),
                }
            }
        }
    };
}
