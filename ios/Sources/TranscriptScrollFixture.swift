#if DEBUG && targetEnvironment(simulator)
import SwiftUI

/// Network-free rendered transcript; never available in device or release builds.
struct TranscriptScrollFixture: View {
    @StateObject private var voice = VoiceOutput()
    @State private var messages = (0..<55).map {
        ChatMessage(role: "assistant", content: "History \($0)\n" +
            String(repeating: "Older conversation content.\n", count: 6))
    } + [ChatMessage(role: "assistant", content: "Growing reply\n")]
    @State private var busy = false
    @State private var generation = 0

    var body: some View {
        VStack {
            HStack {
                Button("Grow reply") { grow() }.accessibilityIdentifier("fixture-grow")
                Button("New message") {
                    messages.append(ChatMessage(role: "user", content: "New message \(generation)"))
                }.accessibilityIdentifier("fixture-new-message")
                Button("Busy") { busy.toggle() }.accessibilityIdentifier("fixture-busy")
            }.font(.caption)
            Text("Generation \(generation)").accessibilityIdentifier("fixture-generation")
            ChatTranscript(messages: messages, busy: busy, images: [], voice: voice)
        }
    }

    private func grow() {
        generation += 1
        let index = messages.count - 1
        let message = messages[index]
        // Same identity and count: reproduces streaming updates, not append-only updates.
        messages[index] = ChatMessage(id: message.id, role: message.role,
            content: message.content + String(repeating: "Streamed text \(generation).\n", count: 30))
    }
}
#endif