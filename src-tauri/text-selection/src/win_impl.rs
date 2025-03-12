use crate::types::{HostHelperTrait, Result, TextSelectionDetectResult};
use windows::Win32::{
    System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL},
    UI::Accessibility::{
        CUIAutomation, IUIAutomation, IUIAutomationTextPattern, UIA_TextPatternId,
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
}

thread_local! {
    static AUTOMATION: IUIAutomation = init_automation();
}

fn init_automation() -> IUIAutomation {
    unsafe {
        log::info!("COM init start");

        // Init COM
        let init = CoInitialize(None);
        if init.is_err() {
            let msg = init.message();
            log::error!("COM init failed: {}", msg);
            panic!("COM init failed");
        }

        // Create IUIAutomation instance
        let auto: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL).unwrap();

        log::info!("COM init success");
        auto
    }
}

/// https://github.dev/pot-app/Selection/blob/master/src/windows.rs
fn get_text_by_automation() -> Result<String> {
    log::info!("get text by automation start");

    AUTOMATION.with(|auto| -> Result<String> {
        log::info!("get auto object {:?}", auto);

        unsafe {
            // Get Focused Element
            let el = auto.GetFocusedElement()?;
            log::info!("get focused element success");

            // Get TextPattern
            let res: IUIAutomationTextPattern = el.GetCurrentPatternAs(UIA_TextPatternId)?;
            log::info!("get text pattern success");

            // Get TextRange Array
            let text_array = res.GetSelection()?;
            log::info!("get text range array success");

            let length = text_array.Length()?;
            log::info!("text range array length: {}", length);

            // Iterate TextRange Array
            let mut target = String::new();

            for i in 0..length {
                let text = text_array.GetElement(i)?;
                let str = text.GetText(-1)?;
                let str = str.to_string();
                target.push_str(&str);
            }

            log::info!("get text success");
            Ok(target.trim().to_string())
        }
    })
}
