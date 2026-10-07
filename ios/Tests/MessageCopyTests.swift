import XCTest
import UIKit
import UniformTypeIdentifiers
@testable import CodeTether

@MainActor
final class MessageCopyTests: XCTestCase {
    private let sample = "# Summary\n\n**Bold** and *italic* [link](https://example.com)\n\n- First\n- Second\n\n```swift\nlet value = 1\n```"

    func testMarkdownIsExactAndNotTruncated() throws {
        let source = sample + String(repeating: "\n**More**", count: 3000)
        let item = try MessageCopyPayload.make(source, format: .markdown)
        XCTAssertEqual(String(data: try XCTUnwrap(item[UTType.utf8PlainText.identifier]), encoding: .utf8), source)
        XCTAssertEqual(item["net.daringfireball.markdown"], Data(source.utf8))
    }

    func testPlainTextRemovesFormattingButPreservesCodeAndWords() throws {
        let item = try MessageCopyPayload.make(sample, format: .plainText)
        let text = String(decoding: try XCTUnwrap(item[UTType.utf8PlainText.identifier]), as: UTF8.self)
        XCTAssertTrue(text.contains("Summary")); XCTAssertTrue(text.contains("First"))
        XCTAssertTrue(text.contains("let value = 1"))
        XCTAssertFalse(text.contains("**Bold**")); XCTAssertFalse(text.contains("```"))
        XCTAssertNil(item[UTType.rtf.identifier])
    }

    func testRichTextIncludesHTMLAndRTFWithBoldAndPlainFallback() throws {
        let item = try MessageCopyPayload.make(sample, format: .richText)
        let html = String(decoding: try XCTUnwrap(item[UTType.html.identifier]), as: UTF8.self)
        XCTAssertTrue(html.contains("<h1>")); XCTAssertTrue(html.contains("<strong>Bold</strong>"))
        XCTAssertTrue(html.contains("href=\"https://example.com\""))
        let rtf = try XCTUnwrap(item[UTType.rtf.identifier])
        let rich = try NSAttributedString(data: rtf, options: [.documentType: NSAttributedString.DocumentType.rtf], documentAttributes: nil)
        let range = (rich.string as NSString).range(of: "Bold")
        XCTAssertNotEqual(range.location, NSNotFound)
        if range.location != NSNotFound {
            let font = rich.attribute(.font, at: range.location, effectiveRange: nil) as? UIFont
            XCTAssertEqual(font?.fontDescriptor.symbolicTraits.contains(.traitBold), true)
        }
        XCTAssertNotNil(item[UTType.utf8PlainText.identifier])
        let board = UIPasteboard.withUniqueName()
        board.setItems([item])
        XCTAssertEqual(board.data(forPasteboardType: UTType.rtf.identifier), rtf)
    }
}