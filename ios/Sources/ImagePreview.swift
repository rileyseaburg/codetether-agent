import SwiftUI

/// Square thumbnail for a pending image attachment with a remove button.
struct ImagePreview: View {
    let image: UIImage
    let onRemove: () -> Void
    var body: some View {
        Image(uiImage: image).resizable().scaledToFill().frame(width: 56, height: 56).clipped()
            .overlay(alignment: .topTrailing) { removeButton }
    }
    private var removeButton: some View {
        Button(action: onRemove) {
            Image(systemName: "xmark.circle.fill").symbolRenderingMode(.palette)
                .foregroundStyle(.white, .black)
        }.accessibilityLabel("Remove attached image")
    }
}
