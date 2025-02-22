#![allow(dead_code)]
use std::{
    any::type_name,
    thread::sleep,
    time::{Duration, Instant},
};

use accessibility::{AXAttribute, AXTextMarkerRange, AXUIElement};
use accessibility_sys::{
    kAXFocusedApplicationAttribute, kAXFocusedUIElementAttribute, kAXRoleAttribute,
    kAXSelectedTextAttribute, kAXTitleAttribute, kAXTrustedCheckOptionPrompt,
    AXIsProcessTrustedWithOptions,
};
use clipboard_rs::{Clipboard, ClipboardContext};
use core_foundation::{
    base::TCFType, boolean::CFBoolean, dictionary::CFDictionary, string::CFString, ConcreteCFType,
};
use rdev::{listen, Button, EventType, Key};
use serde_derive::Serialize;

#[allow(non_upper_case_globals)]
pub const kAXSelectedTextMarkerRangeAttribute: &str = "AXSelectedTextMarkerRange";

pub fn listen_text_selection() {
    {
        // 必须先模拟一次按键，否者 辅助性 获取选中的文本会报错 Ax(-25204)
        let key_event = rdev::EventType::KeyPress(Key::Escape);
        rdev::simulate(&key_event).expect("press");
        std::thread::sleep(std::time::Duration::from_millis(10));
        let key_event = rdev::EventType::KeyRelease(Key::Escape);
        rdev::simulate(&key_event).expect("release");
    }

    unsafe {
        // request accessibility permission
        let prompt_conf_key = CFString::wrap_under_create_rule(kAXTrustedCheckOptionPrompt);
        let conf_dict =
            CFDictionary::from_CFType_pairs(&[(prompt_conf_key, CFBoolean::from(true))]);
        let result = AXIsProcessTrustedWithOptions(conf_dict.as_concrete_TypeRef());

        println!("prompt: {:?}", result);
    }

    #[derive(Debug)]
    struct SelectionEventMark {
        mouse_down_pos: (f64, f64),
        has_mouse_down: bool,
        mouse_down_ts: Instant,
    }

    let mut event_marker = SelectionEventMark {
        mouse_down_pos: (0.0, 0.0),
        has_mouse_down: false,
        mouse_down_ts: Instant::now(),
    };

    // 监听系统事件aaaa
    listen(move |event| {
        match event.event_type {
            EventType::ButtonPress(Button::Left) => {
                event_marker.mouse_down_pos = get_mouse_position();
                event_marker.has_mouse_down = true;
                event_marker.mouse_down_ts = Instant::now();
            }

            EventType::ButtonRelease(Button::Left) => {
                let mut should_check_selection = false;

                if event_marker.has_mouse_down {
                    let now = Instant::now();

                    let is_passed_distance_check =
                        distance(event_marker.mouse_down_pos, get_mouse_position()) > 5.0;

                    if now.duration_since(event_marker.mouse_down_ts) > Duration::from_millis(50)
                        && is_passed_distance_check
                    {
                        should_check_selection = true;
                    }
                }

                if should_check_selection {
                    match has_selected_text() {
                        Ok(val) => match val {
                            SelectedMark::Selected => {
                                println!("detected selected text")
                            }
                            SelectedMark::Text(s) => {
                                // println!("mouse position {:?}", mouse_pos);

                                println!("selected text: {}", s)
                            }
                            SelectedMark::None => {
                                println!("no selected text")
                            }
                        },
                        Err(err) => {
                            println!("err {:?}", err);
                        }
                    }
                }

                event_marker.has_mouse_down = false;
            }
            _ => {}
        }
    })
    .unwrap();
}

pub enum SelectedMark {
    Selected,
    None,
    Text(String),
}

fn has_selected_text() -> Result<SelectedMark, Box<dyn std::error::Error>> {
    let sys_element = AXUIElement::system_wide();
    let focused_app: AXUIElement = get_element_attr(&sys_element, kAXFocusedApplicationAttribute)?;

    let focused_element: AXUIElement =
        get_element_attr(&focused_app, kAXFocusedUIElementAttribute)?;

    let app_name = get_element_attr::<CFString>(&focused_app, kAXTitleAttribute)?;

    if let Ok(selected_text) =
        get_element_attr::<CFString>(&focused_element, kAXSelectedTextAttribute)
    {
        let selected_text = selected_text.to_string();

        if !selected_text.is_empty() {
            return Ok(SelectedMark::Text(selected_text));
        }
    }

    let has_selected_marker_range = has_selected_text_mark_range(&focused_element)?;

    if has_selected_marker_range {
        return Ok(SelectedMark::Selected);
    }

    let str = get_selected_text_by_simulate_copy_action(app_name.to_string().as_str());

    if !str.is_empty() {
        return Ok(SelectedMark::Text(str));
    }

    return Ok(SelectedMark::None);
}

fn has_selected_text_mark_range(
    selected_element: &AXUIElement,
) -> Result<bool, Box<dyn std::error::Error>> {
    let marker_range: AXTextMarkerRange =
        get_element_attr(&selected_element, &kAXSelectedTextMarkerRangeAttribute)?;

    let is_the_same_marker =
        marker_range.start_marker().bytes() == marker_range.end_marker().bytes();

    let has_selected_text = !is_the_same_marker;

    Ok(has_selected_text)
}

fn get_element_attr<T: ConcreteCFType>(
    element: &AXUIElement,
    attr: &str,
) -> Result<T, Box<dyn std::error::Error>> {
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

fn print_available_attrs(element: &AXUIElement, print: bool) -> Vec<String> {
    let attrs = element.attribute_names();

    if let Ok(attrs) = &attrs {
        if print {
            println!("attr: {:?}", attrs);
        }

        return attrs.iter().map(|s| s.to_string()).collect();
    }

    return vec![];
}

fn print_all_attrs(element: &AXUIElement) {
    let attrs = print_available_attrs(&element, false);

    for attr in attrs {
        let val = get_element_attr::<CFString>(&element, &attr.as_str());

        println!("attr: {}, val: {:?}", attr, val)
    }
}

fn print_available_actions(element: &AXUIElement) {
    if let Ok(names) = element.action_names() {
        println!("actions: {:?}", names);
    }
}

fn get_selected_text_by_simulate_copy_action(app_title: &str) -> String {
    // get clipboard data
    let ctx = ClipboardContext::new().unwrap();

    ctx.clear().expect("clear clipboard");

    if !trigger_copy_menu_for_app(app_title) {
        rdev::simulate(&EventType::KeyPress(Key::MetaLeft)).expect("press");
        std::thread::sleep(std::time::Duration::from_millis(50));

        rdev::simulate(&EventType::KeyPress(Key::KeyC)).expect("press");
        std::thread::sleep(std::time::Duration::from_millis(50));

        rdev::simulate(&EventType::KeyRelease(Key::KeyC)).expect("release");
        std::thread::sleep(std::time::Duration::from_millis(50));

        rdev::simulate(&EventType::KeyRelease(Key::MetaLeft)).expect("release");
        std::thread::sleep(std::time::Duration::from_millis(50));

        println!("simulate copy action");
    }

    sleep(Duration::from_millis(50));

    if let Ok(text) = ctx.get_text() {
        return text;
    }

    String::new()
}

fn trigger_copy_menu_for_app(app_name: &str) -> bool {
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

fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let x = p1.0 - p2.0;
    let y = p1.1 - p2.1;

    return (x * x + y * y).sqrt();
}

fn get_mouse_position() -> (f64, f64) {
    use core_graphics::event::CGEvent;
    use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

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
