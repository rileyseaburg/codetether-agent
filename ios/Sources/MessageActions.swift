import SwiftUI

/// Observe audio state only in the action row, not in the entire Markdown message.
struct MessageActions: View {
    let message: ChatMessage
    @ObservedObject var voice: VoiceOutput
    var beforeSpeak: () -> Void = {}
    var onEdit: ((ChatMessage) -> Void)?
    private var active: Bool { voice.activeMessageID == message.id && voice.speaking }
    var body: some View {
        HStack(spacing: 18) {
            Button {
                if active { voice.stop() }
                else { beforeSpeak(); voice.speak(message.content, messageID: message.id) }
            } label: {
                Label(active ? "Stop reading" : "Read aloud", systemImage: active ? "stop.circle.fill" : "speaker.wave.2.fill")
            }
            .disabled(message.content.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
            .accessibilityLabel(active ? "Stop reading this message" : "Read this message aloud with Kokoro")
            .accessibilityIdentifier("message-speaker-\(message.role)")
            MessageCopyMenu(text: message.content)
            if message.role == "user" {
                Button("Edit", systemImage: "pencil") { onEdit?(message) }
                    .disabled(onEdit == nil).accessibilityIdentifier("message-edit")
            }
        }.font(.caption)
    }
}