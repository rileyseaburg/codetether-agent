import XCTest
@testable import CodeTether

final class InlineImageLiveTests: XCTestCase {
    @MainActor
    func testStreamImageSurvivesRevisionWithoutLeakingToNextReply() {
        let chat = ChatModel()
        chat.messages = [ChatMessage(role: "user", content: "Draw")]
        chat.appendReplyImages(["/home/riley/first.png", "/home/riley/first.png"])
        let id = chat.currentReplyID
        chat.upsertReply("Working")
        chat.upsertReply("Done /home/riley/first.png")
        XCTAssertEqual(chat.currentReplyID, id)
        XCTAssertEqual(chat.messages.count, 2)
        XCTAssertEqual(chat.messages.last?.imagePaths, ["/home/riley/first.png"])
        chat.currentReplyID = nil
        chat.messages.append(ChatMessage(role: "user", content: "Next"))
        chat.appendReplyImages(["/home/riley/first.png"])
        chat.upsertReply("Another answer")
        XCTAssertEqual(chat.messages.last?.imagePaths, [])
        chat.appendReplyImages(["/home/riley/second.png"])
        XCTAssertEqual(chat.messages.last?.imagePaths, ["/home/riley/second.png"])
    }

    @MainActor
    func testSnapshotDoesNotDumpOldImagesOntoUnrelatedReply() throws {
        let json = #"{"id":"s","messages":[{"role":"user","content":[{"type":"text","text":"Old"}]},{"role":"tool","content":[{"type":"tool_result","content":"/home/riley/old.png"}]},{"role":"user","content":[{"type":"text","text":"New"}]},{"role":"assistant","content":[{"type":"text","text":"No image"}]}]}"#
        let snapshot = try JSONDecoder().decode(AgentSession.self, from: Data(json.utf8))
        let chat = ChatModel()
        chat.reconcileReplyImages(snapshot, prompt: "New")
        chat.upsertReply("No image")
        XCTAssertEqual(chat.messages.last?.imagePaths, [])
        chat.clear()
        XCTAssertTrue(chat.messages.isEmpty)
        XCTAssertNil(chat.currentReplyID)
    }

    func testPresentationImagesAreNotEncodedInChatRequests() throws {
        let message = ChatMessage(role: "assistant", content: "Reply", imagePaths: ["/home/riley/x.png"])
        let data = try JSONEncoder().encode(message)
        XCTAssertEqual(try JSONDecoder().decode(ChatMessage.self, from: data).imagePaths, [])
        XCTAssertFalse(String(decoding: data, as: UTF8.self).contains("imagePaths"))
    }
}