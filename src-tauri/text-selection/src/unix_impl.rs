use std::any::type_name;

use accessibility::{AXAttribute, AXTextMarkerRange, AXUIElement};
use accessibility_sys::{
    kAXFocusedApplicationAttribute, kAXFocusedUIElementAttribute, kAXSelectedTextAttribute,
    kAXTrustedCheckOptionPrompt, AXIsProcessTrustedWithOptions,
};
use core_foundation::{
    base::TCFType, boolean::CFBoolean, dictionary::CFDictionary, string::CFString, ConcreteCFType,
};
use serde::Serialize;

use crate::types::{HostHelperTrait, Result, TextSelectionDetectResult};

#[allow(non_upper_case_globals)]
pub const kAXSelectedTextMarkerRangeAttribute: &str = "AXSelectedTextMarkerRange";

pub struct HostImpl {
    sys: AXUIElement,
}

thread_local! {
    static AUTOMATION: () = init_automation();
}

fn init_automation() {
    HostImpl::init();
}

impl Default for HostImpl {
    fn default() -> Self {
        AUTOMATION.with(|_| {});

        let sys_element = AXUIElement::system_wide();

        Self { sys: sys_element }
    }
}

impl HostImpl {
    pub fn init() {
        Self::_request_accessibility_access();
        Self::_prepare();
    }

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
            // todo
        }
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

#[allow(dead_code)]
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
