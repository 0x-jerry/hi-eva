use uiautomation::{patterns::UITextPattern, UIAutomation};

use crate::types::{HostHelperTrait, Result, TextSelectionDetectResult};

#[derive(Default)]
pub struct HostImpl;

impl HostHelperTrait for HostImpl {
    fn detect_selected_text(&self) -> Result<TextSelectionDetectResult> {
        let selected_text = get_text_by_automation()?;

        if !selected_text.is_empty() {
            return Ok(TextSelectionDetectResult::Text(selected_text));
        }

        return Ok(TextSelectionDetectResult::None);
    }

    fn get_selected_text(&self) -> Result<String> {
        match self.detect_selected_text() {
            Ok(s) => match s {
                TextSelectionDetectResult::Text(x) => Ok(x),
                _ => Err("NotFound".into()),
            },
            Err(err) => Err(err),
        }
    }
}

fn get_text_by_automation() -> Result<String> {
    log::info!("get text by automation start");

    let auto = UIAutomation::new()?;

    let focused = auto.get_focused_element()?;

    let text = focused.get_pattern::<UITextPattern>()?;

    let selection = text.get_selection()?;

    let result = selection
        .iter()
        .map(|s| s.get_text(-1).unwrap_or_default())
        .collect::<Vec<String>>()
        .join("");

    log::info!("get text by automation end");

    Ok(result)
}
