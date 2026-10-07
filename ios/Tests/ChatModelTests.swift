import XCTest
@testable import CodeTether

@MainActor private final class FakeAgent: AgentTransport {
    var fail = false
    func prompt(sessionID: String, message: String, model: String, status: @escaping (AgentFrame.Event) -> Void) async throws -> AgentReply {
        if fail { throw ClientError.unauthorized }
        return AgentReply(text: "Hello Riley", session_id: sessionID)
    }
}
final class ChatModelTests: XCTestCase {
    @MainActor
    func testSendAppendsReplyAndPreservesHistory() async throws {
        try TokenStore.save("fixture-chat")
        defer { try? TokenStore.remove() }
        StubProtocol.handler = { _ in
            (200, Data(#"{"id":"test-session","messages":[]}"#.utf8))
        }
        let chat = ChatModel(client: stubClient())
        chat.agent = FakeAgent()
        chat.sessionID = "test-session"
        chat.selectedModel = "agent"
        chat.draft = "Hi"
        chat.send()
        _ = await chat.requestTask?.value
        XCTAssertEqual(chat.messages.map(\.content), ["Hi", "Hello Riley"])
        XCTAssertFalse(chat.busy)
        XCTAssertNil(chat.error)
        XCTAssertTrue(chat.draft.isEmpty)
    }

    @MainActor
    func testFailureRestoresDraftWithoutDuplicatingUserMessage() async throws {
        try TokenStore.save("fixture-chat")
        defer { try? TokenStore.remove() }
        StubProtocol.handler = { _ in (401, Data()) }
        let chat = ChatModel(client: stubClient())
        let agent = FakeAgent(); agent.fail = true
        chat.agent = agent; chat.sessionID = "test-session"
        chat.selectedModel = "agent"
        chat.draft = "Keep this message"
        chat.send()
        _ = await chat.requestTask?.value
        XCTAssertTrue(chat.messages.isEmpty)
        XCTAssertEqual(chat.draft, "Keep this message")
        XCTAssertNotNil(chat.error)
        XCTAssertFalse(chat.busy)
    }
}