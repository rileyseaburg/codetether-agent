mod device;
mod dom;
mod eval;
mod lifecycle;
mod navigation;
mod net;
mod tabs;

pub use device::{KeyboardPressRequest, KeyboardTypeRequest, PointerClick};
pub use dom::{
    ClickTextRequest, FillRequest, KeyPressRequest, ScopeRequest, SelectorRequest, ToggleRequest,
    TypeRequest, UploadRequest,
};
pub use eval::EvalRequest;
pub use lifecycle::{ScreenshotRequest, StartRequest, WaitRequest};
pub use navigation::NavigationRequest;
pub use net::{
    AxiosRequest, DiagnoseRequest, FetchRequest, NetworkLogRequest, ReplayRequest, XhrRequest,
};
pub use tabs::{CloseTabRequest, NewTabRequest, TabSelectRequest};
