import Foundation

/// Matches Windows keyboard delivery: a single line, never Enter/Tab or controls.
enum ScreenTypingText {
    static func valid(_ text: String) -> Bool {
        ScreenInput.text(text) && !text.unicodeScalars.contains {
            $0.value < 0x20 || (0x7f...0x9f).contains($0.value)
                || $0.value == 0x2028 || $0.value == 0x2029
        }
    }
}
