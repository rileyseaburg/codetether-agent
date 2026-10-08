import SwiftUI

struct ChatTranscript: View {
    let messages: [ChatMessage]
    let busy: Bool
    let voice: VoiceOutput
    var beforeSpeak: () -> Void = {}
    var onEdit: ((ChatMessage) -> Void)?
    @State private var visibleCount = 40
    @StateObject private var scrolling = TranscriptScrollController()
    private var visible: ArraySlice<ChatMessage> { messages.suffix(visibleCount) }
    var body: some View {
        ZStack(alignment: .bottomTrailing) {
            ScrollView {
                if messages.isEmpty {
                    ContentUnavailableView("Start a conversation", systemImage: "bubble.left.and.bubble.right",
                        description: Text("Ask your CodeTether agent a question or attach an image."))
                        .padding(.top, 70)
                }
                TranscriptWindow(messages: visible, hasEarlier: messages.count > visibleCount,
                    busy: busy, voice: voice, beforeSpeak: beforeSpeak, onEdit: onEdit) {
                        scrolling.following = false
                        visibleCount += 40
                    }
                    .background(TranscriptScrollProbe(controller: scrolling))
            }
            .accessibilityIdentifier("chat-transcript")
            .onChange(of: messages.count) { old, new in
                if !scrolling.following, new > old {
                    visibleCount += new - old
                }
            }
            .scrollDismissesKeyboard(.interactively)
            if !scrolling.following {
                Button { scrolling.resume() } label: {
                    Label("Jump to latest", systemImage: "arrow.down")
                }
                .buttonStyle(.borderedProminent).padding()
                .accessibilityIdentifier("chat-jump-latest")
            }
        }
    }
}