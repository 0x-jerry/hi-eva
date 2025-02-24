#![allow(dead_code)]
use std::{any::type_name, thread::sleep, time::Duration};

use accessibility::{AXAttribute, AXTextMarkerRange, AXUIElement};
use accessibility_sys::{
    kAXFocusedApplicationAttribute, kAXFocusedUIElementAttribute, kAXSelectedTextAttribute,
    kAXTitleAttribute, kAXTrustedCheckOptionPrompt, AXIsProcessTrustedWithOptions,
};
use core_foundation::{
    base::TCFType, boolean::CFBoolean, dictionary::CFDictionary, string::CFString, ConcreteCFType,
};
use core_graphics::{
    event::CGEvent,
    event_source::{CGEventSource, CGEventSourceStateID},
};
use rdev::{EventType, Key};
use serde_derive::Serialize;

use crate::{
    clipboard_helper::ClipboardHostTrait,
    types::{HostHelperTrait, Result, TextSelectionDetectResult},
};

#[allow(non_upper_case_globals)]
pub const kAXSelectedTextMarkerRangeAttribute: &str = "AXSelectedTextMarkerRange";

pub struct HostImpl {
    sys: AXUIElement,
}

impl HostImpl {
    fn _request_accessibility_access() -> bool {
        unsafe {
            // request accessibility permission
            let prompt_conf_key = CFString::wrap_under_create_rule(kAXTrustedCheckOptionPrompt);
            let conf_dict =
                CFDictionary::from_CFType_pairs(&[(prompt_conf_key, CFBoolean::from(true))]);
            let result = AXIsProcessTrustedWithOptions(conf_dict.as_concrete_TypeRef());

            println!("prompt: {:?}", result);

            return result;
        }
    }

    fn _prepare() {
        {
            // 必须先模拟一次按键，否者 辅助性 获取选中的文本会报错 Ax(-25204)
            let key_event = rdev::EventType::KeyPress(Key::Escape);
            rdev::simulate(&key_event).expect("press");
            std::thread::sleep(std::time::Duration::from_millis(10));
            let key_event = rdev::EventType::KeyRelease(Key::Escape);
            rdev::simulate(&key_event).expect("release");
        }
    }
}

impl Default for HostImpl {
    fn default() -> Self {
        Self::_prepare();
        Self::_request_accessibility_access();

        let sys_element = AXUIElement::system_wide();

        Self { sys: sys_element }
    }
}

impl ClipboardHostTrait for HostImpl {
    fn trigger_copy_action(&self) -> Result<()> {
        let focused_app: AXUIElement = get_element_attr(&self.sys, kAXFocusedApplicationAttribute)?;

        let app_title = get_element_attr::<CFString>(&focused_app, kAXTitleAttribute)?.to_string();

        if !trigger_copy_menu_for_app(&app_title) {
            let key_left_meta = Key::MetaLeft;
            let key_c = Key::KeyC;

            rdev::simulate(&EventType::KeyPress(key_left_meta))?;

            rdev::simulate(&EventType::KeyPress(key_c))?;

            sleep(Duration::from_millis(10));

            rdev::simulate(&EventType::KeyRelease(key_left_meta))?;

            rdev::simulate(&EventType::KeyRelease(key_c))?;
        }

        return Ok(());
    }
}

impl HostHelperTrait for HostImpl {
    fn detect_selected_text(&self) -> Result<TextSelectionDetectResult> {
        let focused_app: AXUIElement = get_element_attr(&self.sys, kAXFocusedApplicationAttribute)?;

        let focused_element: AXUIElement =
            get_element_attr(&focused_app, kAXFocusedUIElementAttribute)?;

        let selected_text =
            get_element_attr::<CFString>(&focused_element, kAXSelectedTextAttribute)?.to_string();

        if !selected_text.is_empty() {
            return Ok(TextSelectionDetectResult::Text(selected_text));
        }

        let has_selected_marker_range = has_selected_text_mark_range(&focused_element)?;

        if has_selected_marker_range {
            return Ok(TextSelectionDetectResult::Selected);
        }

        return Ok(TextSelectionDetectResult::None);
    }

    fn get_selected_text(&self) -> Result<String> {
        let focused_app: AXUIElement = get_element_attr(&self.sys, kAXFocusedApplicationAttribute)?;

        let focused_element: AXUIElement =
            get_element_attr(&focused_app, kAXFocusedUIElementAttribute)?;

        let selected_text =
            get_element_attr::<CFString>(&focused_element, kAXSelectedTextAttribute)?;

        return Ok(selected_text.to_string());
    }

    fn get_mouse_position(&self) -> (f64, f64) {
        let event =
            CGEvent::new(CGEventSource::new(CGEventSourceStateID::CombinedSessionState).unwrap());

        let point = match event {
            Ok(event) => {
                let point = event.location();
                return (point.x, point.y);
            }
            Err(_) => (0.0, 0.0),
        };

        point
    }
}

fn has_selected_text_mark_range(selected_element: &AXUIElement) -> Result<bool> {
    let marker_range: AXTextMarkerRange =
        get_element_attr(&selected_element, &kAXSelectedTextMarkerRangeAttribute)?;

    let is_the_same_marker =
        marker_range.start_marker().bytes() == marker_range.end_marker().bytes();

    let has_selected_text = !is_the_same_marker;

    Ok(has_selected_text)
}

fn get_element_attr<T: ConcreteCFType>(element: &AXUIElement, attr: &str) -> Result<T> {
    let attr_value = element.attribute(&AXAttribute::new(&CFString::new(&attr)));

    match attr_value {
        Ok(attr_value) => {
            let raw_value = attr_value.clone();

            if let Some(value) = attr_value.downcast::<T>() {
                return Ok(value);
            };

            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Convert {:?} to type {:?} failed",
                    raw_value,
                    type_name::<T>()
                ),
            )));
        }
        Err(err) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("get attr {} failed: {:?}", attr, err),
            )));
        }
    }
}

fn trigger_copy_menu_for_app(app_name: &str) -> bool {
    // todo, support more app
    let supported_app = ["Chromium", "Code"];
    if !supported_app.contains(&app_name) {
        return false;
    }

    #[derive(Serialize)]
    struct ScriptParams {
        name: String,
    }

    let code = "
        const appName = $params.name
        const app = Application(appName)
        app.activate()

        const sys = Application('System Events')
        const proc = sys.processes.byName(appName)

        const menuBar = proc.menuBars[0]
        const menuBarItem = menuBar.menuBarItems.byName('Edit')
        const menu = menuBarItem.menus[0]
        const menuItem = menu.menuItems.byName('Copy')

        menuItem.click()
    ";

    let script = osascript::JavaScript::new(&code);

    if let Err(err) = script.execute_with_params::<ScriptParams, ()>(ScriptParams {
        name: app_name.to_string(),
    }) {
        println!("Error: {}", err);
        return false;
    }

    return true;
}
