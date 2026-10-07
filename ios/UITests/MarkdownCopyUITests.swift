import XCTest
import UIKit

/// Mocked local: real SwiftUI Markdown view and system clipboard; no server requests.
final class MarkdownCopyUITests: XCTestCase {
    func testResponseRendersAndOffersAllCopyFormats() {
        let app = XCUIApplication()
        app.launchArguments = ["--uitesting", "--markdown-copy-fixture"]
        app.launch()
        XCTAssertTrue(app.staticTexts["Formatted response"].waitForExistence(timeout: 10))
        let copy = app.buttons["message-copy"]
        if !copy.isHittable { app.swipeUp() }
        XCTAssertTrue(copy.waitForExistence(timeout: 5))
        copy.tap()
        for title in ["Rich text", "Plain text", "Markdown"] {
            XCTAssertTrue(app.buttons[title].waitForExistence(timeout: 5))
        }
        app.buttons["Markdown"].tap()
        XCTAssertTrue(app.buttons["Copied"].waitForExistence(timeout: 5))
        copy.tap()
        app.buttons["Plain text"].tap()
        XCTAssertTrue(app.buttons["Copied"].exists)
        copy.tap()
        app.buttons["Rich text"].tap()
        let screenshot = XCTAttachment(screenshot: app.screenshot())
        screenshot.name = "formatted-response-copy-menu"
        screenshot.lifetime = .keepAlways; add(screenshot)
    }
}