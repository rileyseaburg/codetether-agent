import XCTest
@testable import CodeTether

@MainActor
final class NewConversationTests: XCTestCase {
    func testChatAndVoiceNewConversationsRouteToDistinctServerSessions() async throws {
        try TokenStore.save("fixture-new-session")
        let saved = UserDefaults.standard.string(forKey: "agent.session")
        defer { try? TokenStore.remove(); UserDefaults.standard.set(saved, forKey: "agent.session") }
        let first = UUID().uuidString, second = UUID().uuidString
        var creations = 0
        StubProtocol.handler = { request in
            if request.httpMethod == "POST" { creations += 1 }
            let id = creations == 1 ? first : second
            return (200, Data("{\"id\":\"\(id)\",\"messages\":[]}".utf8))
        }
        let chat = ChatModel(client: stubClient()), agent = SessionIsolationAgent()
        chat.agent = agent
        let firstReady = await chat.newConversation()
        XCTAssertTrue(firstReady); XCTAssertEqual(chat.sessionID, first)
        chat.draft = "Typed question"; chat.send(); _ = await chat.requestTask?.value
        XCTAssertEqual(chat.messages.map(\.content), ["Typed question", "Reply: Typed question"])
        let secondReady = await chat.newConversation()
        XCTAssertTrue(secondReady); XCTAssertEqual(chat.sessionID, second)
        XCTAssertTrue(chat.messages.isEmpty)
        _ = await chat.sendAsync("Spoken question")
        XCTAssertEqual(agent.sessions, [first, second])
        XCTAssertEqual(chat.messages.map(\.content), ["Spoken question", "Reply: Spoken question"])
        XCTAssertEqual(creations, 2)
        XCTAssertEqual(UserDefaults.standard.string(forKey: "agent.session"), second)
    }

    func testFailedNewSessionCannotFallBackToPreviousSession() async throws {
        try TokenStore.save("fixture-new-session")
        defer { try? TokenStore.remove() }
        StubProtocol.handler = { _ in (401, Data()) }
        let chat = ChatModel(client: stubClient())
        chat.sessionID = UUID().uuidString
        chat.messages = [ChatMessage(role: "assistant", content: "Old answer")]
        let ready = await chat.newConversation()
        XCTAssertFalse(ready); XCTAssertNil(chat.sessionID)
        XCTAssertTrue(chat.messages.isEmpty); XCTAssertNotNil(chat.error)
        XCTAssertFalse(chat.loading)
    }
}