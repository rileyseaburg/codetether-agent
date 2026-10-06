import XCTest

/// Mocked local UI evidence: real transcript layout, no network or credentials.
final class TranscriptScrollUITests: XCTestCase {
    func testStreamingPausesForHistoryAndResumesAtLatest() {
        let app = XCUIApplication()
        app.launchArguments = ["--uitesting", "--transcript-scroll-fixture"]
        app.launch()
        let bottom = app.staticTexts["fixture-bottom"]
        assertVisible(bottom)
        for _ in 0..<3 {
            app.buttons["fixture-grow"].tap()
            assertVisible(bottom)
        }
        let transcript = app.scrollViews["chat-transcript"]
        transcript.swipeDown()
        transcript.swipeDown()
        let latest = app.buttons["chat-jump-latest"]
        assertVisible(latest)
        XCTAssertFalse(bottom.isHittable, "Reading history must leave the bottom offscreen")
        app.buttons["fixture-grow"].tap()
        XCTAssertTrue(app.staticTexts["Generation 4"].waitForExistence(timeout: 5))
        XCTAssertFalse(bottom.isHittable, "Streaming must not pull a paused reader down")
        capture(app, name: "paused-history")
        latest.tap()
        assertVisible(bottom)
        app.buttons["fixture-grow"].tap()
        assertVisible(bottom)
        app.buttons["fixture-new-message"].tap()
        assertVisible(bottom)
        app.buttons["fixture-busy"].tap()
        assertVisible(bottom)
        capture(app, name: "following-latest")
    }

    private func assertVisible(_ element: XCUIElement) {
        let condition = NSPredicate { _, _ in element.exists && element.isHittable }
        let expectation = XCTNSPredicateExpectation(predicate: condition, object: nil)
        XCTAssertEqual(XCTWaiter.wait(for: [expectation], timeout: 10), .completed)
    }
    private func capture(_ app: XCUIApplication, name: String) {
        let attachment = XCTAttachment(screenshot: app.screenshot())
        attachment.name = name
        attachment.lifetime = .keepAlways
        add(attachment)
    }
}