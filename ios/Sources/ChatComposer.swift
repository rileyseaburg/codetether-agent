import SwiftUI

struct ChatComposer: View {
    @ObservedObject var chat: ChatModel
    var beforeSend: () -> Void = {}
    var body: some View {
        HStack(alignment: .bottom, spacing: 12) {
            TextField("Message CodeTether…", text: $chat.draft, axis: .vertical)
                .lineLimit(1...6).padding(12)
                .background(Color(.secondarySystemBackground), in: RoundedRectangle(cornerRadius: 18))
                .accessibilityIdentifier("chat-input")
            if chat.busy {
                Button { chat.stop() } label: { Image(systemName: "stop.circle.fill").font(.title) }
                    .accessibilityLabel("Stop response")
            } else {
                Button { beforeSend(); chat.send() } label: { Image(systemName: "arrow.up.circle.fill").font(.title) }
                    .disabled(chat.draft.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty && chat.attachments.isEmpty)
                    .accessibilityLabel("Send message").accessibilityIdentifier("chat-send")
            }
        }
        .padding().background(.bar)
    }
}
