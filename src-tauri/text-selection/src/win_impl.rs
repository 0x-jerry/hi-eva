use crate::types::{HostHelperTrait, Result, TextSelectionDetectResult};
use windows::Win32::{
    Foundation::POINT,
    System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL},
    UI::{
        Accessibility::{
            CUIAutomation, IUIAutomation, IUIAutomationTextPattern, UIA_TextPatternId,
        },
        WindowsAndMessaging::GetCursorPos,
    },
};

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

    /// https://github.com/fayez-nazzal/mouse_position/blob/master/src/impl_windows.rs
    fn get_mouse_position(&self) -> (f64, f64) {
        let mut point = POINT { x: 0, y: 0 };
        let result = unsafe { GetCursorPos(&mut point) };

        if result.is_ok() {
            return (point.x as f64, point.y as f64);
        }

        return (0.0, 0.0);
    }
}

/// https://github.dev/pot-app/Selection/blob/master/src/windows.rs
fn get_text_by_automation() -> Result<String> {
    unsafe {
        // Init COM
        let init = CoInitialize(None);
        if init.is_err() {
            let msg = init.message();

            return Err(msg.into());
        }

        // Create IUIAutomation instance
        let auto: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)?;
        // Get Focused Element
        let el = auto.GetFocusedElement()?;
        // Get TextPattern
        let res: IUIAutomationTextPattern = el.GetCurrentPatternAs(UIA_TextPatternId)?;
        // Get TextRange Array
        let text_array = res.GetSelection()?;
        let length = text_array.Length()?;
        // Iterate TextRange Array
        let mut target = String::new();

        for i in 0..length {
            let text = text_array.GetElement(i)?;
            let str = text.GetText(-1)?;
            let str = str.to_string();
            target.push_str(&str);
        }

        Ok(target.trim().to_string())
    }
}
