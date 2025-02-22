use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub enum TextSelectionDetectResult {
    Selected,
    Text(String),
    None,
}

pub(crate) trait HostHelperTrait {
    fn detect_selected_text(&self) -> Result<TextSelectionDetectResult>;
    fn get_selected_text(&self) -> Result<String>;
    fn get_mouse_position(&self) -> (f64, f64);
}
