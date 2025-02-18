use std::any::type_name;

use accessibility::{AXAttribute, AXTextMarkerRange, AXUIElement};
use accessibility_sys::{
    kAXFocusedUIElementAttribute, kAXSelectedTextAttribute, AXIsProcessTrusted,
};
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
                    SelectedMark::Selected(selected_marker) => {
                        println!("has selected text: {}", selected_marker)
                    }
                    SelectedMark::Text(s) => {
                        println!("selected string is {}", s)
                    }
                },
                Err(err) => {
                    println!("err {:?}", err);
                }
            }
        }
    })
    .unwrap();
}

pub enum SelectedMark {
    Selected(bool),
    Text(String),
}

fn has_selected_text() -> Result<SelectedMark, Box<dyn std::error::Error>> {
    let selected_element: AXUIElement =
        get_element_attr(&AXUIElement::system_wide(), kAXFocusedUIElementAttribute)?;

    let selected_text = get_element_attr(&selected_element, kAXSelectedTextAttribute)
        .unwrap_or(CFString::new(""))
        .to_string();

    if !selected_text.is_empty() {
        return Ok(SelectedMark::Text(selected_text));
    }

    let has_selected_marker_range = has_selected_text_mark_range(&selected_element)?;

    return Ok(SelectedMark::Selected(has_selected_marker_range));
}

fn has_selected_text_mark_range(
    selected_element: &AXUIElement,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mark: AXTextMarkerRange =
        get_element_attr(&selected_element, &kAXSelectedTextMarkerRangeAttribute)?;

    let is_the_same_marker = mark.start_marker().bytes() == mark.end_marker().bytes();

    let has_selected_text = !is_the_same_marker;

    Ok(has_selected_text)
}

fn get_element_attr<T: ConcreteCFType>(
    element: &AXUIElement,
    attr: &'static str,
) -> Result<T, Box<dyn std::error::Error>> {
    let attr_value = element.attribute(&AXAttribute::new(&CFString::from_static_string(&attr)));

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
