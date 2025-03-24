use std::any::type_name;

use accessibility::{AXAttribute, AXTextMarkerRange, AXUIElement};
use accessibility_sys::{
    kAXFocusedApplicationAttribute, kAXFocusedUIElementAttribute, kAXSelectedTextAttribute,
    kAXTrustedCheckOptionPrompt, AXIsProcessTrustedWithOptions,
};
use core_foundation::{
    base::TCFType, boolean::CFBoolean, dictionary::CFDictionary, string::CFString, ConcreteCFType,
};
use objc2_app_kit::NSWorkspace;
use serde::Serialize;

use crate::{
    types::{HostHelperTrait, Result},
    SelectionRect,
};

#[allow(non_upper_case_globals)]
pub const kAXSelectedTextMarkerRangeAttribute: &str = "AXSelectedTextMarkerRange";

#[derive(Default)]
pub struct HostImpl;

impl HostHelperTrait for HostImpl {
    fn detect_selection_rect(&self) -> Result<Option<SelectionRect>> {
        let pid = get_frontmost_app().unwrap();
        let focused_app = AXUIElement::application(pid);

        enable_screen_reader_accessibility(&focused_app)?;

        let focused_element: AXUIElement =
            get_element_attr(&focused_app, kAXFocusedUIElementAttribute)?;

        let selected_text =
            get_element_attr::<CFString>(&focused_element, kAXSelectedTextAttribute)?.to_string();

        if !selected_text.is_empty() {
            let mut rect = SelectionRect::default();
            rect.text = Some(selected_text);
            return Ok(Some(rect));
        }

        let has_selected_marker_range = has_selected_text_mark_range(&focused_element)?;

        if has_selected_marker_range {
            return Ok(None);
        }

        return Ok(None);
    }

    fn get_selected_text(&self) -> Result<String> {
        let sys_element = AXUIElement::system_wide();

        let focused_app: AXUIElement =
            get_element_attr(&sys_element, kAXFocusedApplicationAttribute)?;

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
        log::error!("Error: {}", err);
        return false;
    }

    return true;
}

pub fn request_accessibility_access() -> bool {
    unsafe {
        // request accessibility permission
        let prompt_conf_key = CFString::wrap_under_create_rule(kAXTrustedCheckOptionPrompt);
        let conf_dict =
            CFDictionary::from_CFType_pairs(&[(prompt_conf_key, CFBoolean::from(true))]);
        let result = AXIsProcessTrustedWithOptions(conf_dict.as_concrete_TypeRef());

        log::info!("prompt: {:?}", result);

        return result;
    }
}

fn get_frontmost_app() -> Option<i32> {
    unsafe {
        let app = NSWorkspace::sharedWorkspace().frontmostApplication()?;

        let pid = app.processIdentifier();

        Some(pid)
    }
}

fn enable_screen_reader_accessibility(application: &AXUIElement) -> Result<()> {
    #[allow(non_snake_case)]
    let kAXInspectorEnabled = CFString::new("AXInspectorEnabled");
    #[allow(non_snake_case)]
    let kAXEnhancedUserInterface = CFString::new("AXEnhancedUserInterface");
    #[allow(non_snake_case)]
    let kAXManualAccessibility = CFString::new("AXManualAccessibility");

    let bool_true = CFBoolean::true_value().as_CFType();

    application.set_messaging_timeout(5.0)?;
    let _ = application.set_attribute(&AXAttribute::new(&kAXInspectorEnabled), bool_true.clone());
    let _ = application.set_attribute(
        &AXAttribute::new(&kAXEnhancedUserInterface),
        bool_true.clone(),
    );
    let _ = application.set_attribute(
        &AXAttribute::new(&kAXManualAccessibility),
        bool_true.clone(),
    );

    Ok(())
}
