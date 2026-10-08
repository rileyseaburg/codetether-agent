import XCTest

/// Mocked local: exercise real transcript layout while repeatedly toggling Working.
final class WorkingFlickerUITests: XCTestCase {
    func testWorkingFooterDoesNotMoveTranscriptOrFollowingAnchor() {
        let app = XCUIApplication()
        app.launchArguments = ["--uitesting", "--transcript-scroll-fixture"]
        app.launch()
        let bottom = app.staticTexts["fixture-bottom"]
        XCTAssertTrue(bottom.waitForExistence(timeout: 10))
        let settled = XCTNSPredicateExpectation(predicate: NSPredicate { _, _ in bottom.isHittable }, object: nil)
        XCTAssertEqual(XCTWaiter.wait(for: [settled], timeout: 10), .completed)
        let initial = bottom.frame.midY
        for _ in 0..<8 {
            app.buttons["fixture-busy"].tap()
            XCTAssertTrue(bottom.isHittable)
            XCTAssertEqual(bottom.frame.midY, initial, accuracy: 2)
        }
        let transcript = app.scrollViews["chat-transcript"]
        transcript.swipeDown(); transcript.swipeDown()
        let jump = app.buttons["chat-jump-latest"]
        XCTAssertTrue(jump.waitForExistence(timeout: 5))
        app.buttons["fixture-busy"].tap()
        XCTAssertFalse(bottom.isHittable)
        app.buttons["fixture-grow"].tap()
        XCTAssertFalse(bottom.isHittable)
        let screenshot = XCTAttachment(screenshot: app.screenshot())
        screenshot.name = "working-with-history-position-preserved"
        screenshot.lifetime = .keepAlways
        add(screenshot)
    }
}