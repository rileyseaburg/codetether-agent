import SwiftUI

/// Toolbar buttons for `ChatView` (split for file-size limits).
extension ChatView {
    /// Saved chats / New chat / Settings, grouped. Identifiers are
    /// load-bearing for UITests; ToolbarItemGroup gives a ViewBuilder
    /// context where view modifiers are legal.
    var toolbarContent: some ToolbarContent {
        ToolbarItemGroup {
            Button(action: { showHistory = true }) {
                Label("Saved chats", systemImage: "clock.arrow.circlepath")
            }.accessibilityIdentifier("saved-chats").disabled(chat.busy)
            Button(action: { Task { await startNewConversation() } }) {
                Label("New chat", systemImage: "square.and.pencil")
            }.accessibilityIdentifier("new-chat").disabled(chat.loading)
            Button(action: { showSettings = true }) {
                Label("Settings", systemImage: "gearshape")
            }.disabled(chat.busy)
        }
    }
}