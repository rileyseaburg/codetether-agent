import SwiftUI

struct ChatView: View {
    @ObservedObject var connection: ConnectionModel
    @ObservedObject var chat: ChatModel
    // Internal (not private): ChatView+Toolbar binds these sheets.
    @State var showSettings = false
    @StateObject var voiceInput = VoiceInput()
    @StateObject var voiceOutput = VoiceOutput()
    @State var showHistory = false
    @State private var editingMessage: ChatMessage?

    var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                Label("CodeTether agent · tools enabled", systemImage: "wrench.and.screwdriver")
                    .font(.caption).padding(8).accessibilityIdentifier("agent-mode")
                ChatSessionBadge(chat: chat)
                ChatTranscript(messages: chat.messages, busy: chat.busy,
                               voice: voiceOutput, beforeSpeak: { voiceInput.stop() },
                               onEdit: { voiceInput.stop(); voiceOutput.stop(); editingMessage = $0 })
                    .id(chat.conversationID)
                if let error = chat.error {
                    Text(error).font(.footnote).foregroundStyle(.red).padding(.horizontal)
                }
                ModelSelectorRow(chat: chat)
                AttachmentBar(chat: chat)
                VoiceControls(chat: chat, input: voiceInput, output: voiceOutput)
                ChatComposer(chat: chat, beforeSend: { voiceInput.stop(); voiceOutput.stop() })
            }
            .navigationTitle("CodeTether Chat")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar { toolbarContent }
            .sheet(item: $editingMessage) { MessageEditSheet(chat: chat, message: $0) }
            .sheet(isPresented: $showHistory) { ConversationListView(chat: chat) }
            .sheet(isPresented: $showSettings, onDismiss: { Task { await chat.restoreSession() } }) {
                SettingsView(model: connection)
            }
            .task { await connection.start(); await chat.restoreSession(); await chat.loadModels() }
            .onChange(of: chat.conversationID) { _, _ in voiceInput.stop(); voiceOutput.stop() }
            .onDisappear { voiceInput.stop(); voiceOutput.stop() }
            .onChange(of: chat.voiceModeActive) { _, active in
                if active { voiceInput.stop(); voiceOutput.stop() }
            }
            .onChange(of: chat.replyForSpeech) { _, text in
                if let text, voiceOutput.enabled, chat.shouldReadChatReply {
                    voiceInput.stop(); voiceOutput.speak(text, messageID: chat.currentReplyID)
                }
            }
        }
    }
}