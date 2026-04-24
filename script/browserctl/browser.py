from .session_controls import BrowserSessionControls
from .session_core import BrowserSessionCore
from .session_dom import BrowserSessionDom
from .session_eval import BrowserSessionEval
from .session_input import BrowserSessionInput
from .session_lifecycle import BrowserSessionLifecycle
from .session_media import BrowserSessionMedia
from .session_navigation import BrowserSessionNavigation
from .session_scope import BrowserSessionScope
from .session_tabs import BrowserSessionTabs


class BrowserSession(
    BrowserSessionCore,
    BrowserSessionScope,
    BrowserSessionLifecycle,
    BrowserSessionNavigation,
    BrowserSessionDom,
    BrowserSessionEval,
    BrowserSessionControls,
    BrowserSessionMedia,
    BrowserSessionInput,
    BrowserSessionTabs,
):