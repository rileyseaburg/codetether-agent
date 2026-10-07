#if DEBUG && targetEnvironment(simulator)
import SwiftUI

/// Network-free UI fixture for Markdown layout and the real copy menu.
struct MarkdownCopyFixture: View {
    @StateObject private var voice = VoiceOutput()
    static let text = """
    # Formatted response

    **Bold text**, *italic text*, and a [link](https://example.com).

    - First item
    - Second item

    > A quoted answer.

    | Name | Value |
    | --- | --- |
    | Format | Markdown |

    ```swift
    let greeting = "Hello"
    ```
    """
    var body: some View {
        ScrollView { MessageBubble(message: ChatMessage(role: "assistant", content: Self.text), voice: voice) }
    }
}
#endif