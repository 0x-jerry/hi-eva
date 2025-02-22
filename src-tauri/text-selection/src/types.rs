use std::io::Error;

pub struct TextSelectionTool;

pub enum DetectSelectionResult {
    Selected,
    Text(String),
    None,
}

pub trait TextSelectionAction {
    fn detect_selected_text() -> Result<DetectSelectionResult, Error>;
    fn get_selected_text() -> Result<String, Error>;
}
