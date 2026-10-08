import SwiftUI

/// Editing creates a saved branch and regenerates from this prompt; the original stays intact.
struct MessageEditSheet: View {
    @ObservedObject var chat: ChatModel
    let message: ChatMessage
    @Environment(\.dismiss) private var dismiss
    @State private var text: String
    @State private var saving = false
    @State private var error: String?
    init(chat: ChatModel, message: ChatMessage) {
        self.chat = chat; self.message = message
        _text = State(initialValue: message.content)
    }
    var body: some View {
        NavigationStack {
            VStack(alignment: .leading, spacing: 12) {
                Text("Save creates a new conversation from this point. The original stays in Saved chats.")
                    .font(.footnote).foregroundStyle(.secondary)
                TextEditor(text: $text).accessibilityIdentifier("message-edit-text")
                if let error { Text(error).font(.footnote).foregroundStyle(.red) }
                if saving { ProgressView("Saving edited conversation…") }
            }.padding().navigationTitle("Edit message")
            .toolbar {
                ToolbarItem(placement: .cancellationAction) { Button("Cancel") { dismiss() }.disabled(saving) }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Save & resend") {
                        saving = true; error = nil
                        Task {
                            defer { saving = false }
                            do { try await chat.editAndResend(message, text: text); dismiss() }
                            catch { self.error = error.localizedDescription }
                        }
                    }.disabled(saving || text.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
                        .accessibilityIdentifier("message-edit-save")
                }
            }.interactiveDismissDisabled(saving)
        }
    }
}