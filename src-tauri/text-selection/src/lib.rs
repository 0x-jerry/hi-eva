pub mod types;
pub mod utils;

mod clipboard_helper;

#[cfg(windows)]
mod win_impl;

use std::{
    result,
    time::{Duration, Instant},
};

use clipboard_helper::ClipboardHostTrait;
use rdev::{Button, EventType, ListenError};
pub use types::*;

#[derive(Debug)]
struct SelectionEventMark {
    mouse_down_pos: (f64, f64),
    mouse_down: bool,
    mouse_down_ts: Instant,
}

pub struct ListenResult {
    pub selected_text: String,
    pub mouse_position: (f64, f64),
}

pub fn listen<T: Fn(ListenResult) -> () + 'static>(listener: T) -> result::Result<(), ListenError> {
    let host = {
        #[cfg(windows)]
        win_impl::HostImpl::default()
    };

    let mut event_marker = SelectionEventMark {
        mouse_down_pos: (0.0, 0.0),
        mouse_down: false,
        mouse_down_ts: Instant::now(),
    };

    //
    rdev::listen(move |event| {
        match event.event_type {
            EventType::ButtonPress(Button::Left) => {
                event_marker.mouse_down_pos = host.get_mouse_position();
                event_marker.mouse_down = true;
                event_marker.mouse_down_ts = Instant::now();
            }

            EventType::ButtonRelease(Button::Left) => {
                let mut should_check_selection = false;

                let current_mouse_pos = host.get_mouse_position();

                if event_marker.mouse_down {
                    let now = Instant::now();

                    let is_passed_distance_check =
                        utils::distance(event_marker.mouse_down_pos, current_mouse_pos) > 5.0;

                    if now.duration_since(event_marker.mouse_down_ts) > Duration::from_millis(50)
                        && is_passed_distance_check
                    {
                        should_check_selection = true;
                    }
                }

                if should_check_selection {
                    match detect_selected_text(&host) {
                        Ok(val) => match val {
                            TextSelectionDetectResult::Selected => {
                                println!("detected selected text")
                            }
                            TextSelectionDetectResult::Text(s) => {
                                // println!("mouse position {:?}", current_mouse_pos);

                                println!("selected text: {}", s)
                            }
                            TextSelectionDetectResult::None => {
                                println!("no selected text")
                            }
                        },
                        Err(err) => {
                            println!("err {:?}", err);
                        }
                    }
                }

                event_marker.mouse_down = false;
            }
            _ => {}
        }
    })
}

fn detect_selected_text<T: ClipboardHostTrait + HostHelperTrait>(
    host: &T,
) -> Result<TextSelectionDetectResult> {
    let selected_text = host.detect_selected_text()?;

    match selected_text {
        TextSelectionDetectResult::None => {
            let text_from_clipboard = clipboard_helper::get_selected_text_from_clipboard(host)?;

            if !text_from_clipboard.is_empty() {
                return Ok(TextSelectionDetectResult::Text(text_from_clipboard));
            }

            return Ok(TextSelectionDetectResult::None);
        }
        _ => return Ok(selected_text),
    }
}
