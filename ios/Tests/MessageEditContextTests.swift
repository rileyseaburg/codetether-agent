import XCTest
@testable import CodeTether

final class MessageEditContextTests: XCTestCase {
    func testTargetUsesRawServerIndexAndRetainsImageAndPDFInstructions() throws {
        let raw = "Question\n\nUser attached image files: [\"/home/riley/x.png\"]\nUse the image tool.\n\nUser attached PDF documents: [\"/home/riley/x.pdf\"]\nRead the PDF."
        let text = try JSONEncoder().encode(raw)
        let json = "{\"id\":\"s\",\"messages\":[{\"role\":\"assistant\",\"content\":[{\"type\":\"text\",\"text\":\"Welcome\"}]},{\"role\":\"user\",\"content\":[{\"type\":\"text\",\"text\":\(String(decoding: text, as: UTF8.self))}]}]}"
        let snapshot = try JSONDecoder().decode(AgentSession.self, from: Data(json.utf8))
        let visible = snapshot.transcript
        let target = try MessageEditContext.find(visible[1], in: visible, snapshot: snapshot)
        XCTAssertEqual(target.index, 1)
        XCTAssertEqual(target.original, raw)
        XCTAssertTrue(target.attachmentSuffix.contains("x.png"))
        XCTAssertTrue(target.attachmentSuffix.contains("x.pdf"))
        XCTAssertFalse(target.attachmentSuffix.contains("Question"))
    }

    func testChangedServerMessageRejectsEditInsteadOfTargetingWrongTurn() throws {
        let json = #"{"id":"s","messages":[{"role":"user","content":[{"type":"text","text":"Different"}]}]}"#
        let snapshot = try JSONDecoder().decode(AgentSession.self, from: Data(json.utf8))
        let message = ChatMessage(role: "user", content: "Original")
        XCTAssertThrowsError(try MessageEditContext.find(message, in: [message], snapshot: snapshot))
    }
}