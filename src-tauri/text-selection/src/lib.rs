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
struct SelectionState {
    mouse_down_pos: (f64, f64),
    mouse_down_ts: Instant,
    last_click_ts: Option<Instant>,
    last_click_pos: (f64, f64),
    last_check_ts: Instant,
}

pub fn listen<T: 'static + TextSelectionHandler + Send>(text_selection_handler: T) -> Result<()> {
    #[cfg(target_os = "macos")]
    unix_impl::request_accessibility_access();

    let state = Mutex::new(SelectionState {
        mouse_down_pos: (0.0, 0.0),
        mouse_down_ts: Instant::now(),
        last_click_ts: None,
        last_click_pos: (0.0, 0.0),
        last_check_ts: Instant::now(),
    });

    let mut mouse = mouce::Mouse::new();

    let result = mouse.hook(Box::new(move |event| {
        let mut state = state.try_lock().unwrap();

        handle_mouse_event(event, &mut state, &text_selection_handler);
    }));

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

fn handle_mouse_event<T: 'static + TextSelectionHandler + Send>(
    event: &MouseEvent,
    state: &mut SelectionState,
    text_selection_handler: &T,
) {
    #[cfg(windows)]
    let host = win_impl::HostImpl::default();
    #[cfg(unix)]
    let host = unix_impl::HostImpl::default();

    match event {
        MouseEvent::Press(MouseButton::Left) => {
            state.mouse_down_pos = text_selection_handler.get_cursor_position();
            state.mouse_down_ts = Instant::now();
            text_selection_handler.on_mouse_down();
        }
        MouseEvent::Release(MouseButton::Left) => {
            let mut should_check_selection = false;

            let current_mouse_pos = text_selection_handler.get_cursor_position();

            let now = Instant::now();
            let maybe_click = utils::distance(state.mouse_down_pos, current_mouse_pos) < 5.0;

            if maybe_click {
                if let Some(last_click_ts) = state.last_click_ts {
                    let db_click_max_delay_check_ms = 500;

                    let maybe_double_click = now.duration_since(last_click_ts)
                        < Duration::from_millis(db_click_max_delay_check_ms)
                        && utils::distance(state.last_click_pos, current_mouse_pos) < 5.0;

                    if maybe_double_click {
                        should_check_selection = true;
                        state.last_click_ts = None;
                    } else {
                        state.last_click_ts = Some(now);
                        state.last_click_pos = current_mouse_pos.clone();
                    }
                } else {
                    state.last_click_ts = Some(now);
                    state.last_click_pos = current_mouse_pos.clone();
                }
            } else {
                should_check_selection = true;
                state.last_click_ts = None;
            }

            if should_check_selection {
                let should_check_again =
                    now.duration_since(state.last_check_ts) > Duration::from_millis(100);

                if should_check_again {
                    log::info!("check selection start");
                    match host.detect_selected_text() {
                        Ok(val) => match val {
                            TextSelectionDetectResult::Selected => {
                                text_selection_handler.on_selection_change(Some(ListenResult {
                                    selected_text: String::default(),
                                }));
                            }
                            TextSelectionDetectResult::Text(s) => {
                                text_selection_handler
                                    .on_selection_change(Some(ListenResult { selected_text: s }));
                            }
                            TextSelectionDetectResult::None => {
                                text_selection_handler.on_selection_change(None);
                            }
                        },
                        Err(err) => {
                            log::error!("detect selected text error {:?}", err);
                        }
                    }
                    log::info!("check selection end");

                    state.last_check_ts = now;
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
