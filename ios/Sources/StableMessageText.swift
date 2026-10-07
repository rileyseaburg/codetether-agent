import SwiftUI
import MarkdownUI

/// Parse markdown only when a completed message changes, not on audio/status updates.
struct StableMessageText: View, Equatable {
    let text: String
    var body: some View {
        Markdown(text)
            .markdownTheme(.gitHub)
            .markdownImageProvider(ChatMarkdownImages())
            .markdownInlineImageProvider(ChatMarkdownImages())
            .markdownBlockStyle(\.codeBlock) { block in
                ScrollView(.horizontal) { block.label.padding(10) }
                    .background(Color(.tertiarySystemBackground), in: RoundedRectangle(cornerRadius: 8))
            }
            .markdownBlockStyle(\.table) { block in
                ScrollView(.horizontal) { block.label.fixedSize(horizontal: true, vertical: false) }
            }
            .textSelection(.enabled)
            .frame(maxWidth: .infinity, alignment: .leading)
    }
}