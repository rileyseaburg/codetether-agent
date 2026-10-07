import XCTest
@testable import CodeTether

@MainActor
final class StaleConversationTests: XCTestCase {
    func testNewChatWhileBusyRejectsOldReplyEventsAndCleanup() async throws {
        try TokenStore.save("fixture-isolation")
        defer { try? TokenStore.remove() }
        let oldID = UUID().uuidString, newID = UUID().uuidString
        StubProtocol.handler = { _ in (200, Data("{\"id\":\"\(newID)\",\"messages\":[]}".utf8)) }
        let chat = ChatModel(client: stubClient()), agent = SessionIsolationAgent()
        let started = expectation(description: "Old request running")
        agent.onFirstStarted = { started.fulfill() }
        chat.agent = agent; chat.sessionID = oldID
        chat.draft = "Old question"; chat.send()
        let oldRequest = chat.requestTask
        await fulfillment(of: [started], timeout: 3)
        XCTAssertTrue(chat.busy)
        let ready = await chat.newConversation()
        XCTAssertTrue(ready); XCTAssertEqual(chat.sessionID, newID)
        chat.draft = "New question"; chat.send()
        let newRequest = chat.requestTask
        let event = #"{"kind":"tool.metadata","payload":{"metadata":{"saved_path":"/home/riley/old.png"}}}"#
        agent.oldEvents?(try JSONDecoder().decode(AgentFrame.Event.self, from: Data(event.utf8)))
        agent.pending?.resume(returning: AgentReply(text: "Old late answer", session_id: oldID))
        _ = await oldRequest?.value
        _ = await newRequest?.value
        XCTAssertEqual(agent.sessions, [oldID, newID])
        XCTAssertEqual(chat.messages.map(\.content), ["New question", "Reply: New question"])
        XCTAssertTrue(chat.messages.flatMap(\.imagePaths).isEmpty)
        XCTAssertNil(chat.error); XCTAssertEqual(chat.draft, "")
    }

    func testClearInvalidatesEvenAnActiveVoiceTurn() {
        let chat = ChatModel()
        let turn = chat.beginTurn("Voice question", owner: .voice)
        XCTAssertTrue(chat.busy)
        chat.clear()
        XCTAssertFalse(chat.busy); XCTAssertNil(chat.activeTurnID)
        XCTAssertNotEqual(chat.activeTurnID, turn?.id)
        XCTAssertTrue(chat.messages.isEmpty)
    }
}