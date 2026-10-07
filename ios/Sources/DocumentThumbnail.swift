import SwiftUI

/// Placeholder tile for a pending PDF attachment with a remove button.
struct DocumentThumbnail: View {
    let onRemove: () -> Void
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8).fill(Color.secondary.opacity(0.2))
            Image(systemName: "doc.richtext").font(.title2).foregroundStyle(.secondary)
        }.frame(width: 56, height: 56)
        .overlay(alignment: .topTrailing) { removeButton }
    }
    private var removeButton: some View {
        Button(action: onRemove) {
            Image(systemName: "xmark.circle.fill").symbolRenderingMode(.palette)
                .foregroundStyle(.white, .black)
        }.accessibilityLabel("Remove attached PDF")
    }
}
