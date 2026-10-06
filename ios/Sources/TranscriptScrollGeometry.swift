import UIKit

enum TranscriptScrollGeometry {
    static func bottomOffset(_ view: UIScrollView) -> CGFloat {
        max(-view.adjustedContentInset.top,
            view.contentSize.height - view.bounds.height + view.adjustedContentInset.bottom)
    }

    static func isNearBottom(_ view: UIScrollView) -> Bool {
        bottomOffset(view) - view.contentOffset.y <= 64
    }
}