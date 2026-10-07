import UIKit

/// Width-clamped sizing for the composer text view so it never exceeds the offered width.
enum ComposerSizing {
    static let maximumLines = 6

    static func size(thatFits view: UITextView, proposedWidth: CGFloat) -> CGSize {
        let fit = view.sizeThatFits(CGSize(width: proposedWidth, height: .greatestFiniteMagnitude))
        let cap = lineCap(for: view)
        return CGSize(width: proposedWidth, height: min(ceil(fit.height), cap))
    }

    private static func lineCap(for view: UITextView) -> CGFloat {
        let line = view.font?.lineHeight ?? UIFont.systemFont(ofSize: 17).lineHeight
        let insets = view.textContainerInset.top + view.textContainerInset.bottom
        return ceil(CGFloat(maximumLines) * line + insets)
    }
}
