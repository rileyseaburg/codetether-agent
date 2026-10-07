import SwiftUI

/// Make server-session ownership visible in both conversation interfaces.
struct ChatSessionBadge: View {
    @ObservedObject var chat: ChatModel
    var body: some View {
        Text(chat.loading ? "Creating new session…" : "Session: \(chat.sessionID.map { String($0.prefix(8)) } ?? "new")")
            .font(.caption.monospaced()).foregroundStyle(.secondary)
            .accessibilityLabel(chat.loading ? "Creating new session" : "Session \(chat.sessionID ?? "new")")
            .accessibilityIdentifier("chat-session-id")
    }
}