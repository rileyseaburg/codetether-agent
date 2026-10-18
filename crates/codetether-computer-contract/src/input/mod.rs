//! Input types for the computer_use tool.

mod action;
mod mode;
mod ocr;
mod request;

pub use action::ComputerUseAction;
pub use mode::InputMode;
pub use ocr::OcrInput;
pub use request::ComputerUseInput;
