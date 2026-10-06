import SwiftUI

struct ChatTranscript: View {
    let messages: [ChatMessage]
    let busy: Bool
    var images: [String] = []
    @ObservedObject var voice: VoiceOutput
    var beforeSpeak: () -> Void = {}
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
                LazyVStack(alignment: .leading, spacing: 18) {
                    if messages.count > visibleCount {
                        Button("Load earlier messages") {
                            scrolling.following = false
                            visibleCount += 40
                        }
                    }
                    ForEach(visible) { message in
                        MessageBubble(message: message, voice: voice, beforeSpeak: beforeSpeak)
                    }
                    ForEach(images, id: \.self) { AgentImageView(path: $0) }
                    if busy { ProgressView("Working…").padding(.vertical).accessibilityIdentifier("chat-thinking") }
                    Color.clear.frame(height: 1).id("bottom")
#if DEBUG && targetEnvironment(simulator)
                    if CommandLine.arguments.contains("--transcript-scroll-fixture") {
                        Text("Bottom of transcript").accessibilityIdentifier("fixture-bottom")
                    }
#endif
                }.padding().background(TranscriptScrollProbe(controller: scrolling))
            }
            .accessibilityIdentifier("chat-transcript")
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