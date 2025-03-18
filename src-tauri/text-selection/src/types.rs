use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub(crate) trait HostHelperTrait {
    fn detect_selection_rect(&self) -> Result<Option<SelectionRect>>;
    fn get_selected_text(&self) -> Result<String>;
}

#[derive(Debug, Default)]
pub struct SelectionRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
