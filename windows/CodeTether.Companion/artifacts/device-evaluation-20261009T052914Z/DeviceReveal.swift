import XCTest

extension DeviceTypingEvaluation {
    /// Scroll only the phone form; never change the Windows input target.
    func reveal(_ element: XCUIElement, in app: XCUIApplication) {
        if element.exists && element.isHittable { return }
        for _ in 0..<7 {
            app.swipeUp()
            if element.exists && element.isHittable { return }
        }
    }
}