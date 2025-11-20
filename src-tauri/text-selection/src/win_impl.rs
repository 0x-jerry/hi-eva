use std::{cmp, i32, thread};

use windows::Win32::{
    Foundation::RECT,
    System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL},
    UI::Accessibility::{
        CUIAutomation, IUIAutomation, IUIAutomationTextPattern, UIA_TextPatternId,
    },
};

use crate::{
    types::{HostHelperTrait, Result},
    SelectionRect,
};

#[derive(Default)]
pub struct HostImpl;

impl HostHelperTrait for HostImpl {
    fn detect_selection_rect(&self) -> Result<Option<SelectionRect>> {
        // use other thread to get text, avoid break this thread
        let auto_handle = thread::spawn(|| {
            let selected_text = get_selection_range().unwrap();
            selected_text
        });

        let selection = auto_handle.join();

        if selection.is_err() {
            log::error!("detect selected text error: {:?}", selection.err());
            return Ok(None);
        }

        Ok(selection.unwrap())
    }

    fn get_selected_text(&self) -> Result<String> {
        let s = get_selected_text()?;

        Ok(s.unwrap_or_default())
    }
}

fn get_selection_range() -> Result<Option<SelectionRect>> {
    log::info!("get selection by automation start");

    let mut rect: Option<SelectionRect> = None;

    unsafe {
        let init = CoInitialize(None);

        log::info!("init");
        if init.is_err() {
            let msg = init.message();
            log::error!("COM init failed: {}", msg);
            panic!("COM init failed");
        }

        let auto: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)?;

        log::info!("init automation");

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

        if length <= 0 {
            return Ok(None);
        }

        let mut selected_text = String::new();

        for i in 0..length {
            let text = text_array.GetElement(i)?;
            let range = text.GetBoundingRectangles()?;

            let s = text.GetText(i32::MAX)?.to_string();
            selected_text.push_str(s.as_str());

            let mut data_ptr: *mut RECT = std::ptr::null_mut();
            let length = auto.SafeArrayToRectNativeArray(range, &mut data_ptr)?;
            let rects = std::slice::from_raw_parts(data_ptr, length as usize).to_vec();

            log::info!("length is {}", length);

            for item in rects {
                log::info!("rect is {:?}", item);

                match rect {
                    Some(ref mut rect) => {
                        rect.left = cmp::min(rect.left, item.left);
                        rect.top = cmp::min(rect.top, item.top);
                        rect.right = cmp::max(rect.right, item.right);
                        rect.bottom = cmp::min(rect.bottom, item.bottom);
                    }
                    None => {
                        rect = Some(SelectionRect {
                            text: None,
                            left: item.left,
                            top: item.top,
                            right: item.right,
                            bottom: item.bottom,
                        });
                    }
                }
            }
        }

        if let Some(ref mut rect) = rect {
            rect.text = Some(selected_text);
        }
    }

    log::info!("get selection range by automation end");

    Ok(rect)
}

fn get_selected_text() -> Result<Option<String>> {
    log::info!("get selection by automation start");

    #[allow(unused_assignments)]
    let mut result: Option<String> = None;

    unsafe {
        let init = CoInitialize(None);

        log::info!("init");
        if init.is_err() {
            let msg = init.message();
            log::error!("COM init failed: {}", msg);
            panic!("COM init failed");
        }

        let auto: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)?;

        log::info!("init automation");

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

        if length <= 0 {
            return Ok(None);
        }

        let mut s = String::new();

        for i in 0..length {
            let text = text_array.GetElement(i)?;
            let text_str = text.GetText(i32::MAX)?;
            let str = text_str.to_string();

            s.push_str(str.as_str());
        }

        result = Some(s);
    }

    log::info!("get selection range by automation end");

    Ok(result)
}
