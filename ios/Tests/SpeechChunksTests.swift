import XCTest
@testable import CodeTether

final class SpeechChunksTests: XCTestCase {
    func testLongResponsesRespectKokoroUTF16Limit() {
        let text = String(repeating: "Hello Riley 🌍. ", count: 200)
        let chunks = SpeechChunks.split(text)
        XCTAssertGreaterThan(chunks.count, 1)
        XCTAssertTrue(chunks.allSatisfy { !$0.isEmpty && $0.utf16.count <= 380 })
    }
    func testLongSingleWordSplitsWithoutBreakingEmoji() {
        XCTAssertTrue(SpeechChunks.split(String(repeating: "🌍", count: 500)).allSatisfy { $0.utf16.count <= 380 })
    }
    func testCodeFencesAreNotReadAsCode() {
        XCTAssertEqual(SpeechChunks.split("Hi ```secret code``` Riley"), ["Hi Code block. Riley"])
        XCTAssertTrue(SpeechChunks.split("   ").isEmpty)
    }
}
