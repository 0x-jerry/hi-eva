use std::io::Error;

pub enum TextSelectionDetectResult {
    Selected,
    Text(String),
    None,
}

pub trait TextSelectionHostTrait {
    fn detect_selected_text(&self) -> Result<TextSelectionDetectResult, Error>;
    fn get_selected_text(&self) -> Result<String, Error>;
}
