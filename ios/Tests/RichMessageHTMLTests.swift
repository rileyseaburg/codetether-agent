import XCTest
@testable import CodeTether

final class RichMessageHTMLTests: XCTestCase {
    func testDropsActiveResourcesAndPreservesSafeFormattingAndLinks() throws {
        let html = try RichMessageHTML.document(#"<p style="background:url(https://tracker.invalid)">Hello <strong>bold</strong><img src="https://tracker.invalid/a"><iframe src="https://tracker.invalid"></iframe><a href="javascript:alert(1)" onclick="alert(1)">bad</a><a href="https://example.com">safe</a></p>"#)
        XCTAssertTrue(html.contains("<strong>bold</strong>"))
        XCTAssertTrue(html.contains("href=\"https://example.com\""))
        XCTAssertFalse(html.contains("tracker.invalid"))
        XCTAssertFalse(html.contains("javascript:"))
        XCTAssertFalse(html.contains("onclick"))
        XCTAssertFalse(html.contains("<iframe"))
    }

    func testCodeEntitiesAreNotTurnedIntoExecutableTags() throws {
        let html = try RichMessageHTML.document("<pre><code>&lt;img src=x&gt;</code></pre>")
        XCTAssertTrue(html.contains("&lt;img src=x&gt;"))
        XCTAssertFalse(html.contains("<img"))
    }
    func testCopyFormatsAreTheThreeRequestedChoices() {
        XCTAssertEqual(MessageCopyFormat.allCases.map(\.rawValue), ["Rich text", "Plain text", "Markdown"])
    }
}