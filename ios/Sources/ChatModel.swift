import Foundation

@MainActor
final class ChatModel: ObservableObject {
    @Published var draft = ""
    @Published var messages: [ChatMessage] = []
    @Published var models: [String] = []
    @Published var selectedModel = "" {
        didSet { UserDefaults.standard.set(selectedModel, forKey: "chat.model") }
    }
    @Published var busy = false
    @Published var loading = false
    @Published var error: String?
    @Published var agentStatus = ""
    @Published var replyForSpeech: String?
    @Published var tools: [String] = []
    @Published var attachments: [UserImage] = []
    @Published var sessionID = UserDefaults.standard.string(forKey: "agent.session")
    var agent: AgentTransport = AgentSocket()
    @Published var generatedImages: [String] = []
    let client: ServerClient
    var requestTask: Task<Void, Never>?
    var activeAssistantItem: String?
    var completedItems: [String] = []
    var currentReplyID: UUID?
    init(client: ServerClient = ServerClient()) { self.client = client }

    func stop() { requestTask?.cancel() }
}
