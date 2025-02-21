#![allow(dead_code)]
use std::{any::type_name, thread::sleep, time::Duration};

use accessibility::{AXAttribute, AXTextMarkerRange, AXUIElement};
use accessibility_sys::{
    kAXFocusedApplicationAttribute, kAXFocusedUIElementAttribute, kAXFocusedWindowAttribute,
    kAXSelectedTextAttribute, kAXTitleAttribute, AXIsProcessTrusted,
};
use clipboard_rs::{Clipboard, ClipboardContext};
use core_foundation::{string::CFString, ConcreteCFType};
use rdev::{listen, Button, EventType, Key};

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

    let mut mouse_pos = (0 as f64, 0 as f64);

    // 监听系统事件aaaa
    listen(move |event| {
        if let EventType::ButtonRelease(Button::Left) = event.event_type {
            let is_trusted = unsafe { AXIsProcessTrusted() };

            if !is_trusted {
                println!("AXIsProcessTrusted is false");
                return;
            }

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

        if let EventType::MouseMove { x, y } = event.event_type {
            mouse_pos = (x, y);
            // println!("MouseMove x:{} y:{}", x, y);
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
    let focused_win: AXUIElement = get_element_attr(&focused_app, kAXFocusedWindowAttribute)?;

    let focused_element: AXUIElement =
        get_element_attr(&focused_app, kAXFocusedUIElementAttribute)?;

    let app_name = get_element_attr::<CFString>(&focused_app, kAXTitleAttribute)?;

    // print_available_actions(&focused_element);
    // let _ = focused_element.perform_action(&CFString::from("AXShowMenu"));

    let selected_text = get_element_attr(&focused_element, kAXSelectedTextAttribute)
        .unwrap_or(CFString::new(""))
        .to_string();

    if !selected_text.is_empty() {
        return Ok(SelectedMark::Text(selected_text));
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

    // rdev::simulate(&EventType::KeyPress(Key::MetaLeft)).expect("press");
    // std::thread::sleep(std::time::Duration::from_millis(50));

    // rdev::simulate(&EventType::KeyPress(Key::KeyC)).expect("press");
    // std::thread::sleep(std::time::Duration::from_millis(50));

    // rdev::simulate(&EventType::KeyRelease(Key::KeyC)).expect("release");
    // std::thread::sleep(std::time::Duration::from_millis(50));

    // rdev::simulate(&EventType::KeyRelease(Key::MetaLeft)).expect("release");
    // std::thread::sleep(std::time::Duration::from_millis(50));

    // println!("simulate copy action");

    trigger_copy_menu_for_app(app_title);
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

    let code = format!(
        "
        const appName = '{}'
        const app = Application(appName)
        app.activate()

        const sys = Application('System Events')
        const proc = sys.processes.byName(appName)

        const menuBar = proc.menuBars[0]
        const menuBarItem = menuBar.menuBarItems.byName('Edit')
        const menu = menuBarItem.menus[0]
        const menuItem = menu.menuItems.byName('Copy')

        menuItem.click()
        ",
        app_name
    );

    let script = osascript::JavaScript::new(&code);

    if let Err(err) = script.execute::<()>() {
        println!("Error: {}", err);
        return false;
    }

    return true;
}
