import SwiftUI

/// Exactly measured content for the bounded transcript window; no lazy height estimates.
struct TranscriptWindow: View {
    let messages: ArraySlice<ChatMessage>
    let hasEarlier: Bool
    let busy: Bool
    let voice: VoiceOutput
    let beforeSpeak: () -> Void
    var onEdit: ((ChatMessage) -> Void)?
    let loadEarlier: () -> Void
    var body: some View {
        VStack(alignment: .leading, spacing: 18) {
            if hasEarlier { Button("Load earlier messages", action: loadEarlier) }
            ForEach(messages) { message in
                MessageBubble(message: message, voice: voice, beforeSpeak: beforeSpeak, onEdit: busy ? nil : onEdit)
            }
            TranscriptWorkingIndicator(busy: busy)
            Color.clear.frame(height: 1).id("bottom")
#if DEBUG && targetEnvironment(simulator)
            if CommandLine.arguments.contains("--transcript-scroll-fixture") {
                Text("Bottom of transcript").accessibilityIdentifier("fixture-bottom")
            }
#endif
        }.padding()
    }
}
