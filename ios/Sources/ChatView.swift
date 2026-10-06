import SwiftUI

struct ChatView: View {
    @ObservedObject var connection: ConnectionModel
    @StateObject private var chat = ChatModel()
    @State private var showSettings = false
    @StateObject private var voiceInput = VoiceInput()
    @StateObject private var voiceOutput = VoiceOutput()
    @State private var confirmClear = false
    @State private var showHistory = false

    var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                Label("CodeTether agent · tools enabled", systemImage: "wrench.and.screwdriver")
                    .font(.caption).padding(8).accessibilityIdentifier("agent-mode")
                ChatTranscript(messages: chat.messages, busy: chat.busy, images: chat.generatedImages,
                               voice: voiceOutput, beforeSpeak: { voiceInput.stop() })
                    .id(chat.sessionID ?? "new-conversation")
                if let error = chat.error {
                    Text(error).font(.footnote).foregroundStyle(.red).padding(.horizontal)
                }
                AttachmentBar(chat: chat)
                VoiceControls(chat: chat, input: voiceInput, output: voiceOutput)
                ChatComposer(chat: chat, beforeSend: { voiceInput.stop(); voiceOutput.stop() })
            }
            .navigationTitle("CodeTether Chat")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                Button("Saved chats", systemImage: "clock.arrow.circlepath") { showHistory = true }
                    .disabled(chat.busy).accessibilityIdentifier("saved-chats")
                Button("New chat", systemImage: "square.and.pencil") { confirmClear = true }.disabled(chat.busy)
                    .accessibilityIdentifier("new-chat")
                Button("Settings", systemImage: "gearshape") { showSettings = true }.disabled(chat.busy)
            }
            .confirmationDialog("Clear this conversation?", isPresented: $confirmClear) {
                Button("New chat", role: .destructive) { chat.clear() }.accessibilityIdentifier("confirm-new-chat")
            }
            .sheet(isPresented: $showHistory) { ConversationListView(chat: chat) }
            .sheet(isPresented: $showSettings, onDismiss: { Task { await chat.restoreSession() } }) {
                SettingsView(model: connection)
            }
            .task { await connection.start(); await chat.restoreSession() }
            .onChange(of: chat.sessionID) { _, _ in voiceOutput.stop() }
            .onChange(of: chat.replyForSpeech) { _, text in
                if let text, voiceOutput.enabled {
                    voiceInput.stop(); voiceOutput.speak(text, messageID: chat.currentReplyID)
                }
            }
        }
    }
}
