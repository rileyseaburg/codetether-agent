import XCTest

/// Attaches to the existing paired app; never launches, stops, or resets it.
final class DeviceTypingEvaluation: XCTestCase {
    func testExistingSessionModelTyping() throws {
        continueAfterFailure = false
        let app = XCUIApplication(bundleIdentifier: "run.codetether.ios")
        guard app.state == .runningForeground || app.state == .runningBackground else {
            XCTFail("Existing iPhone application is not running; no launch attempted"); return
        }
        app.activate()
        app.tabBars.buttons["Screen"].tap()
        let question = app.descendants(matching: .any).matching(identifier: "screen-question").firstMatch
        reveal(question, in: app)
        XCTAssertTrue(question.exists, "Existing paired session has no Ask field; no new session created")
        let draft = question.value as? String ?? ""
        XCTAssertTrue(draft.isEmpty || draft == "What should AI check on screen now?",
                      "Existing draft preserved; no request sent")
        question.tap()
        question.typeText("Type exactly CT-LIVE-052914 into the already focused Notepad document only. Do not click, press Enter, or submit. If the focused field is not Notepad, do not type.")
        let ask = app.buttons["screen-ask"]
        reveal(ask, in: app)
        XCTAssertTrue(ask.isEnabled, "Existing session does not permit a fresh request")
        ask.tap()
        print("DEVICE_EVAL request_submitted_once marker=CT-LIVE-052914")
        let analysis = app.descendants(matching: .any).matching(identifier: "screen-analysis").firstMatch
        for _ in 0..<5 { app.swipeDown() }
        reveal(analysis, in: app)
        let finished = NSPredicate { _, _ in
            let text = analysis.label
            return text.contains("CT-LIVE-052914") && text.contains("Typing queued for Windows")
                || text.contains("No typing queued") || text.contains("Typing was not queued")
                || app.staticTexts["Analysis error"].exists
        }
        let result = XCTWaiter.wait(for: [XCTNSPredicateExpectation(predicate: finished, object: nil)], timeout: 150)
        let queued = analysis.label.contains("CT-LIVE-052914") && analysis.label.contains("Typing queued for Windows")
        print("DEVICE_EVAL model_finished=\(result == .completed) typing_queued=\(queued) insertion_not_proven=true")
        XCTAssertEqual(result, .completed, "No terminal model result observed; never retry typing automatically")
        XCTAssertTrue(queued, "Live model did not queue the requested text; inspect device without resending")
        // Leave the phone, Windows, pairing, and live session untouched at teardown.
    }
}