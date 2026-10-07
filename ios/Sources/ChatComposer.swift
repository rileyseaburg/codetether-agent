import SwiftUI

struct ChatComposer: View {
    @ObservedObject var chat: ChatModel
    var beforeSend: () -> Void = {}
    var body: some View {
        HStack(alignment: .bottom, spacing: 12) {
            ImagePasteControl { image in
                if let image { chat.attachCapture(image) }
                else { chat.error = "Could not paste that image. Copy the screenshot again and retry." }
            }
            .frame(width: 44, height: 44)
            .disabled(chat.busy || chat.attachments.count >= UserImage.maximumAttachments)
            .accessibilityHint("Copy a screenshot, then tap to attach it")
            PasteAwareTextField(text: $chat.draft, onImagePaste: { chat.attachCapture($0) },
                                accessibilityIdentifier: "chat-input", placeholder: "Message CodeTether…")
            if chat.busy {
                Button { chat.stop() } label: { Image(systemName: "stop.circle.fill").font(.title) }
                    .accessibilityLabel("Stop response")
            } else {
                Button { beforeSend(); chat.send() } label: { Image(systemName: "arrow.up.circle.fill").font(.title) }
                    .disabled(chat.draft.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty && chat.attachments.isEmpty)
                    .accessibilityLabel("Send message").accessibilityIdentifier("chat-send")
            }
        }
        .padding().background(.bar).disabled(chat.loading)
    }
}