import Foundation
import Combine

/// App-owned session state survives tab switches; it never owns screenshot pixels.
@MainActor
final class ScreenModel: ObservableObject {
    @Published var prompt = "Describe my Windows screen and help me with the task shown."
    @Published var interval = 30
    @Published var session: ScreenSession?
    @Published var response = ScreenResponse()
    @Published var activeModel = ""
    @Published var creating = false
    @Published var stopping = false
    @Published var connected = false
    @Published var error: String?
    @Published var notice = "Create a session to pair your Windows device."
    @Published var retryBlocked = false
    let questions = ScreenQuestionState()
    let replies = ScreenReplyState()
    let client: ScreenNetworking
    let token: ScreenToken
    let delay: ScreenDelay
    var active = false
    var generation = UUID()
    var streamTask: Task<Void, Never>?
    var expiryTask: Task<Void, Never>?

    init(client: ScreenNetworking? = nil, token: @escaping ScreenToken = { try TokenStore.read() },
         delay: @escaping ScreenDelay = { try await Task.sleep(nanoseconds: $0) }) {
        self.client = client ?? ScreenClient()
        self.token = token
        self.delay = delay
    }
    var locked: Bool { session != nil || creating || stopping }
    func credential() throws -> String {
        guard let value = try token() else { throw ClientError.missingToken }
        return value
    }
    func invalidateStream() {
        generation = UUID()
        streamTask?.cancel()
        questions.cancel()
        replies.cancel()
        connected = false
    }
    deinit { streamTask?.cancel(); expiryTask?.cancel() }
}
