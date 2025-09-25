use std::{any::type_name, ptr::NonNull};

use objc2_app_kit::NSWorkspace;
use objc2_application_services::{
    kAXTrustedCheckOptionPrompt, AXError, AXIsProcessTrustedWithOptions, AXTextMarkerRange,
    AXUIElement,
};
use objc2_core_foundation::{CFBoolean, CFDictionary, CFRetained, CFString, CFType, ConcreteType};
use serde::Serialize;

use crate::{
    types::{HostHelperTrait, Result},
    SelectionRect,
};

#[allow(non_upper_case_globals)]
pub const kAXSelectedTextMarkerRangeAttribute: &str = "AXSelectedTextMarkerRange";

#[derive(Default)]
pub struct HostImpl;

#[allow(non_upper_case_globals)]
const kAXFocusedUIElementAttribute: &str = "AXFocusedUIElement";
#[allow(non_upper_case_globals)]
const kAXSelectedTextAttribute: &str = "AXSelectedText";

impl HostHelperTrait for HostImpl {
    fn detect_selection_rect(&self) -> Result<Option<SelectionRect>> {
        let pid = get_frontmost_app().unwrap();
        let focused_app = unsafe { AXUIElement::new_application(pid) };

        enable_screen_reader_accessibility(&focused_app)?;

        let focused_element =
            get_element_attr::<AXUIElement>(&focused_app, kAXFocusedUIElementAttribute)?;

        let selected_text =
            get_element_attr::<CFString>(&focused_element, &"AXSelectedText")?.to_string();

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
        let pid = get_frontmost_app().unwrap();
        let focused_app = unsafe { AXUIElement::new_application(pid) };

        enable_screen_reader_accessibility(&focused_app)?;

        let focused_element =
            get_element_attr::<AXUIElement>(&focused_app, kAXFocusedUIElementAttribute)?;

        let selected_text =
            get_element_attr::<CFString>(&focused_element, kAXSelectedTextAttribute)?;

        return Ok(selected_text.to_string());
    }
}

fn has_selected_text_mark_range(selected_element: &AXUIElement) -> Result<bool> {
    let marker_range = get_element_attr::<AXTextMarkerRange>(
        &selected_element,
        &kAXSelectedTextMarkerRangeAttribute,
    )?;

    let is_the_same_marker = unsafe {
        marker_range
            .start_marker()
            .byte_ptr()
            .eq(&marker_range.end_marker().byte_ptr())
    };

    let has_selected_text = !is_the_same_marker;

    Ok(has_selected_text)
}

fn get_element_attr<T: ConcreteType>(element: &AXUIElement, attr: &str) -> Result<CFRetained<T>> {
    unsafe {
        // Prepare output slot
        let mut raw: *const CFType = std::ptr::null();
        let slot = NonNull::new(&mut raw as *mut *const CFType).unwrap();
        let cf_attr = CFString::from_str(attr);

        let err = element.copy_attribute_value(&cf_attr, slot);

        if err == AXError::Success {
            if let Some(nonnull) = NonNull::new(raw as *mut CFType) {
                let t = CFRetained::from_raw(nonnull);
                let v = t.downcast::<T>().unwrap();

                return Ok(v);
            } else {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Convert {:?} to type {:?} failed", attr, type_name::<T>()),
                )));
            }
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Convert {:?} to type {:?} failed", attr, type_name::<T>()),
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
        let options =
            CFDictionary::from_slices(&[kAXTrustedCheckOptionPrompt], &[CFBoolean::new(true)]);

        let result = AXIsProcessTrustedWithOptions(Some(options.as_opaque()));

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
    let kAXInspectorEnabled = CFString::from_str("AXInspectorEnabled");
    #[allow(non_snake_case)]
    let kAXEnhancedUserInterface = CFString::from_str("AXEnhancedUserInterface");
    #[allow(non_snake_case)]
    let kAXManualAccessibility = CFString::from_str("AXManualAccessibility");

    let bool_true = CFBoolean::new(true);

    unsafe {
        application.set_messaging_timeout(5.0);
        let _ = application.set_attribute_value(&kAXInspectorEnabled, bool_true);
        let _ = application.set_attribute_value(&kAXEnhancedUserInterface, bool_true);
        let _ = application.set_attribute_value(&kAXManualAccessibility, bool_true);
    }

    Ok(())
}
