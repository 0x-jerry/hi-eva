pub mod types;
pub mod utils;

#[cfg(unix)]
mod unix_impl;
#[cfg(windows)]
mod win_impl;

use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

use mouce::{
    common::{MouseButton, MouseEvent},
    MouseActions,
};
pub use types::*;

#[derive(Debug)]
struct SelectionEventMark {
    mouse_down_pos: (f64, f64),
    mouse_down_ts: Instant,
    last_click_ts: Option<Instant>,
    last_click_pos: (f64, f64),
}

pub fn listen<T: 'static + TextSelectionHandler + Send>(text_selection_handler: T) -> Result<()> {
    let event_marker = Mutex::new(SelectionEventMark {
        mouse_down_pos: (0.0, 0.0),
        mouse_down_ts: Instant::now(),
        last_click_ts: None,
        last_click_pos: (0.0, 0.0),
    });

    let mut mouse = mouce::Mouse::new();

    let result = mouse.hook(Box::new(move |event| {
        handle_mouse_event(event, &event_marker, &text_selection_handler);
    }));

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

fn handle_mouse_event<T: 'static + TextSelectionHandler + Send>(
    event: &MouseEvent,
    event_marker: &Mutex<SelectionEventMark>,
    text_selection_handler: &T,
) {
    #[cfg(windows)]
    let host = win_impl::HostImpl::default();
    #[cfg(unix)]
    let host = unix_impl::HostImpl::default();

    match event {
        MouseEvent::Press(MouseButton::Left) => {
            let mut event_marker = event_marker.try_lock().unwrap();

            event_marker.mouse_down_pos = host.get_mouse_position();
            event_marker.mouse_down_ts = Instant::now();
        }
        MouseEvent::Release(MouseButton::Left) => {
            let mut event_marker = event_marker.try_lock().unwrap();

            let mut should_check_selection = false;

            let current_mouse_pos = host.get_mouse_position();

            let maybe_click = utils::distance(event_marker.mouse_down_pos, current_mouse_pos) < 5.0;

            if maybe_click {
                let now = Instant::now();

                if let Some(last_click_ts) = event_marker.last_click_ts {
                    let db_click_max_delay_check_ms = 500;

                    let maybe_double_click = now.duration_since(last_click_ts)
                        < Duration::from_millis(db_click_max_delay_check_ms)
                        && utils::distance(event_marker.last_click_pos, current_mouse_pos) < 5.0;

                    if maybe_double_click {
                        should_check_selection = true;
                        event_marker.last_click_ts = None;
                    } else {
                        event_marker.last_click_ts = Some(now);
                        event_marker.last_click_pos = current_mouse_pos.clone();
                    }
                } else {
                    event_marker.last_click_ts = Some(now);
                    event_marker.last_click_pos = current_mouse_pos.clone();
                }
            } else {
                should_check_selection = true;
                event_marker.last_click_ts = None;
            }

            if should_check_selection {
                match host.detect_selected_text() {
                    Ok(val) => match val {
                        TextSelectionDetectResult::Selected => {
                            text_selection_handler.on_selection_change(Some(ListenResult {
                                selected_text: String::default(),
                                mouse_position: host.get_mouse_position(),
                            }));
                        }
                        TextSelectionDetectResult::Text(s) => {
                            text_selection_handler.on_selection_change(Some(ListenResult {
                                selected_text: s,
                                mouse_position: host.get_mouse_position(),
                            }));
                        }
                        TextSelectionDetectResult::None => {
                            text_selection_handler.on_selection_change(None);
                        }
                    },
                    Err(err) => {
                        println!("err {:?}", err);
                    }
                }
            } else {
                text_selection_handler.on_selection_change(None);
            }
        }
        _ => {}
    }
}

pub fn get_selected_text() -> Result<String> {
    #[cfg(windows)]
    let host = win_impl::HostImpl::default();
    #[cfg(unix)]
    let host = unix_impl::HostImpl::default();

    let selected_result = host.get_selected_text()?;

    return Ok(selected_result);
}

pub fn get_mouse_pos() -> (f64, f64) {
    #[cfg(windows)]
    let host = win_impl::HostImpl::default();
    #[cfg(unix)]
    let host = unix_impl::HostImpl::default();

    host.get_mouse_position()
}
