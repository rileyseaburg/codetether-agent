import SwiftUI

/// Voice New chat uses the same new-server-session operation as the Chat toolbar.
struct VoiceSessionScreen: View {
    @ObservedObject var chat: ChatModel
    @ObservedObject var loop: VoiceLoop
    @ObservedObject var mic: VoiceMicEngine
    @Binding var creating: Bool
    @State private var resumeAfterPicker = false
    var body: some View {
        VStack(spacing: 4) {
            HStack {
                ChatSessionBadge(chat: chat)
                Spacer()
                Button("New chat", systemImage: "square.and.pencil") {
                    creating = true
                    loop.stop()
                    let owner = loop.turnID
                    Task {
                        let ready = await chat.newConversation()
                        if ready, owner == loop.turnID, chat.voiceModeActive { await loop.start() }
                        creating = false
                    }
                }.disabled(creating || chat.loading).accessibilityIdentifier("voice-new-chat")
            }.padding(.horizontal)
            if let error = chat.error { Text(error).font(.caption).foregroundStyle(.red) }
            ModelSelectorRow(chat: chat, onPresent: {
                resumeAfterPicker = loop.phase != .idle
                loop.stop()
            }, onDismiss: {
                if resumeAfterPicker, chat.voiceModeActive {
                    Task { if chat.voiceModeActive { await loop.start() } }
                }
                resumeAfterPicker = false
            })
            .disabled(creating || chat.loading)
            .accessibilityHint("Backend model for the next voice turn")
            VoiceModeView(loop: loop, mic: mic).disabled(creating || chat.loading)
        }
    }
}