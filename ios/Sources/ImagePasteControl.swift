import SwiftUI

/// A system paste button: grants clipboard access through an explicit user gesture.
struct ImagePasteControl: UIViewRepresentable {
    let onImage: (UIImage?) -> Void

    func makeUIView(context: Context) -> ImagePasteTarget {
        let view = ImagePasteTarget()
        view.onImage = onImage
        return view
    }

    func updateUIView(_ view: ImagePasteTarget, context: Context) {
        view.onImage = onImage
        view.isUserInteractionEnabled = context.environment.isEnabled
        view.alpha = context.environment.isEnabled ? 1 : 0.4
    }
}