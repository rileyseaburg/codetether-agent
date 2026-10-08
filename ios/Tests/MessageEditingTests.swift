import XCTest
@testable import CodeTether

@MainActor
final class MessageEditingTests: XCTestCase {
    func testEditForksOriginalAndSendsRevisedPromptToNewSession() async throws {
        try TokenStore.save("fixture-edit")
        defer { try? TokenStore.remove() }
        let original = UUID().uuidString, branch = UUID().uuidString
        let snapshot = "{\"id\":\"\(original)\",\"messages\":[{\"role\":\"user\",\"content\":[{\"type\":\"text\",\"text\":\"Original question\"}]},{\"role\":\"assistant\",\"content\":[{\"type\":\"text\",\"text\":\"Old answer\"}]}]}"
        struct Request: Decodable { let before_message: Int; let expected_text: String }
        var fork: Request?
        StubProtocol.handler = { request in
            if request.httpMethod == "POST" {
                fork = try JSONDecoder().decode(Request.self, from: RequestBody.data(request))
                return (200, Data("{\"id\":\"\(branch)\",\"messages\":[]}".utf8))
            }
            return (200, Data(snapshot.utf8))
        }
        let chat = ChatModel(client: stubClient()), agent = SessionIsolationAgent()
        chat.agent = agent; chat.sessionID = original
        let message = ChatMessage(role: "user", content: "Original question")
        chat.messages = [message, ChatMessage(role: "assistant", content: "Old answer")]
        try await chat.editAndResend(message, text: "Revised question")
        _ = await chat.requestTask?.value
        XCTAssertEqual(fork?.before_message, 0); XCTAssertEqual(fork?.expected_text, "Original question")
        XCTAssertEqual(chat.sessionID, branch); XCTAssertEqual(agent.sessions, [branch])
        XCTAssertEqual(chat.messages.map(\.content), ["Revised question", "Reply: Revised question"])
    }

    func testFailedEditLeavesOriginalConversationUntouched() async throws {
        try TokenStore.save("fixture-edit")
        defer { try? TokenStore.remove() }
        StubProtocol.handler = { _ in (409, Data()) }
        let chat = ChatModel(client: stubClient()), id = UUID().uuidString
        chat.sessionID = id
        let message = ChatMessage(role: "user", content: "Original")
        chat.messages = [message]
        do { try await chat.editAndResend(message, text: "Changed"); XCTFail("Expected conflict") }
        catch { }
        XCTAssertEqual(chat.sessionID, id)
        XCTAssertEqual(chat.messages.first?.content, "Original")
        XCTAssertFalse(chat.loading)
    }
}
