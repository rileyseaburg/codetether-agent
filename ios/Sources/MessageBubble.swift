import SwiftUI

struct MessageBubble: View {
    let message: ChatMessage
    @ObservedObject var voice: VoiceOutput
    var beforeSpeak: () -> Void = {}
    @State private var expanded = false
    private var active: Bool { voice.activeMessageID == message.id && voice.speaking }
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
            Button {
                if active { voice.stop() }
                else { beforeSpeak(); voice.speak(message.content, messageID: message.id) }
            } label: {
                Label(active ? "Stop reading" : "Read aloud", systemImage: active ? "stop.circle.fill" : "speaker.wave.2.fill")
                    .font(.caption)
            }
            .disabled(message.content.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
            .accessibilityLabel(active ? "Stop reading this message" : "Read this message aloud with Kokoro")
            .accessibilityIdentifier("message-speaker-\(message.role)")
        }
        .padding(14).frame(maxWidth: .infinity, alignment: .leading)
        .background(message.role == "user" ? Color.accentColor.opacity(0.1) : Color(.secondarySystemBackground),
                    in: RoundedRectangle(cornerRadius: 16))
    }
}
