use crate::{
    types::{Result, TextSelectionDetectResult, TextSelectionHostTrait},
    utils::ClipboardHostTrait,
};
use rdev::{EventType, Key};
use std::{thread::sleep, time::Duration};
use windows::Win32::{
    System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL},
    UI::Accessibility::{
        CUIAutomation, IUIAutomation, IUIAutomationTextPattern, UIA_TextPatternId,
    },
};

#[derive(Default)]
pub struct HostImpl;

impl ClipboardHostTrait for HostImpl {
    fn trigger_copy_action(&self) -> Result<()> {
        //
        let left_ctrl = Key::ControlLeft;
        let key_c = Key::KeyC;

        rdev::simulate(&EventType::KeyPress(left_ctrl))?;

        rdev::simulate(&EventType::KeyPress(key_c))?;

        sleep(Duration::from_millis(10));

        rdev::simulate(&EventType::KeyRelease(left_ctrl))?;

        rdev::simulate(&EventType::KeyRelease(key_c))?;

        Ok(())
    }
}

impl TextSelectionHostTrait for HostImpl {
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
    unsafe {
        // Init COM
        let _ = CoInitialize(None);
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
