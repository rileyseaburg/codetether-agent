import SwiftUI

struct ConversationListView: View {
    @ObservedObject var chat: ChatModel
    @StateObject private var list = ConversationListModel()
    @Environment(\.dismiss) private var dismiss
    @State private var search = ""
    var body: some View {
        NavigationStack {
            List {
                Button("New conversation", systemImage: "square.and.pencil") {
                    Task { if await chat.newConversation() { dismiss() } }
                }.disabled(chat.loading)
                ForEach(list.conversations.filter { search.isEmpty || ($0.title ?? "").localizedCaseInsensitiveContains(search) }) { item in
                    Button {
                        Task { await chat.resume(item.id); dismiss() }
                    } label: {
                        VStack(alignment: .leading, spacing: 5) {
                            HStack {
                                Text(item.title ?? "Untitled conversation").lineLimit(2).foregroundStyle(.primary)
                                if chat.sessionID == item.id { Image(systemName: "checkmark.circle.fill") }
                            }
                            Text("\(item.message_count) messages · \(item.updated_at.prefix(10))")
                                .font(.caption).foregroundStyle(.secondary)
                        }
                    }.accessibilityIdentifier("conversation-\(item.id)")
                }
                if list.loading { ProgressView("Loading conversations…") }
                if let error = list.error { Text(error).foregroundStyle(.red) }
                if list.hasMore && !list.loading {
                    Button("Load older conversations") { Task { await list.load() } }
                }
            }
            .navigationTitle("Saved chats").searchable(text: $search, prompt: "Search loaded conversations")
            .refreshable { await list.load(reset: true) }
            .toolbar { Button("Done") { dismiss() } }
            .task { await list.load(reset: true) }
        }
    }
}