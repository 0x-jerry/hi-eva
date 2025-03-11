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

#[derive(Debug)]
pub struct ListenResult {
    pub selected_text: String,
    pub mouse_position: (f64, f64),
}

pub trait TextSelectionHandler {
    fn on_selection_change(&self, result: Option<ListenResult>);
    fn on_mouse_down(&self);
}
