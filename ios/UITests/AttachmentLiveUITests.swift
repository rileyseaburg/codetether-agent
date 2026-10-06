import XCTest

final class AttachmentLiveUITests: XCTestCase {
    func testUserCanOpenPhotoPicker() {
        let app = XCUIApplication()
        app.launchArguments = ["--uitesting"]
        app.launch()
        let button = app.buttons["add-image"]
        XCTAssertTrue(button.waitForExistence(timeout: 30))
        button.tap()
        let cancel = app.buttons["Cancel"].firstMatch
        XCTAssertTrue(cancel.waitForExistence(timeout: 20))
        cancel.tap()
        XCTAssertTrue(app.buttons["add-image"].waitForExistence(timeout: 10))
    }
}
