import Foundation

struct ConversationSummary: Decodable, Identifiable {
    let id: String
    let title: String?
    let updated_at: String
    let message_count: Int
}

@MainActor
final class ConversationListModel: ObservableObject {
    @Published var conversations: [ConversationSummary] = []
    @Published var loading = false
    @Published var hasMore = true
    @Published var error: String?
    private var offset = 0
    func load(reset: Bool = false) async {
        guard !loading else { return }
        loading = true
        defer { loading = false }
        if reset { offset = 0; conversations = []; hasMore = true }
        do {
            guard let token = try TokenStore.read() else { throw ClientError.missingToken }
            let page: [ConversationSummary] = try await ServerClient().get("api/session?limit=50&offset=\(offset)", token: token)
            let known = Set(conversations.map(\.id))
            conversations.append(contentsOf: page.filter { !known.contains($0.id) })
            offset += page.count
            hasMore = page.count == 50
            error = nil
        } catch { self.error = "Unable to load saved conversations. Try again." }
    }
}
