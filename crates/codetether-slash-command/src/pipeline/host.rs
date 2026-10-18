//! Execution supplied by the native command host.
use super::{SimpleCommand, TailCommand};
use crate::routing::LeadingCommand;
use std::future::Future;
/// Executes command operations while shared policy owns sequencing and aliases.
pub trait Host {
    /// Execute a leading command and consume the command line.
    fn leading(&mut self, kind: LeadingCommand, rest: &str) -> impl Future<Output = ()>;
    /// Handle a Ralph subcommand, returning false for monitor fall-through.
    fn ralph(&mut self, rest: &str) -> impl Future<Output = bool>;
    /// Handle swarm launch/control, returning false for monitor fall-through.
    fn swarm(&mut self, rest: &str) -> bool;
    /// Handle a session command if recognized.
    fn session(&mut self, normalized: &str) -> impl Future<Output = bool>;
    /// Execute a classified simple command.
    fn simple(&mut self, kind: SimpleCommand) -> impl Future<Output = ()>;
    /// Execute a trailing agent or relay command.
    fn tail(&mut self, kind: TailCommand, rest: &str) -> impl Future<Output = ()>;
    /// Display the existing unknown-command status.
    fn unknown(&mut self, normalized: &str);
}
