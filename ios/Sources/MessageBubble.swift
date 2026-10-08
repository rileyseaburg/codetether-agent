import SwiftUI

struct MessageBubble: View {
    let message: ChatMessage
    let voice: VoiceOutput
    var beforeSpeak: () -> Void = {}
    var onEdit: ((ChatMessage) -> Void)?
    @State private var expanded = false
    private var displayedText: String {
        expanded ? message.content : String(message.content.prefix(12000))
    }
    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text(message.role == "user" ? "You" : "CodeTether")
                .font(.caption.weight(.semibold)).foregroundStyle(.secondary)
            StableMessageText(text: displayedText).equatable()
                .accessibilityIdentifier(message.role == "assistant" ? "assistant-message" : "user-message")
            if message.content.count > 12000 && !expanded {
                Button("Show full message") { expanded = true }
            }
            ForEach(message.imagePaths, id: \.self) { path in
                AgentImageView(path: path).accessibilityIdentifier("message-image-\(message.id)-\(path)")
            }
            MessageActions(message: message, voice: voice, beforeSpeak: beforeSpeak, onEdit: onEdit)
        }
        .padding(14).frame(maxWidth: .infinity, alignment: .leading)
        .background(message.role == "user" ? Color.accentColor.opacity(0.1) : Color(.secondarySystemBackground),
                    in: RoundedRectangle(cornerRadius: 16))
    }
}