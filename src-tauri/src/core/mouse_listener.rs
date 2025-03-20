use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

use mouce::{
    common::{MouseButton, MouseEvent},
    MouseActions,
};
use text_selection::SelectionRect;

use super::VerticalMoveDir;

pub trait MouseExtTrait {
    fn on_selection_change(&self, result: Option<SelectionResult>);
    fn on_mouse_down(&self);
    fn on_mouse_move(&self);
    fn get_cursor_position(&self) -> (f64, f64);
}

#[derive(Debug, Default)]
pub struct SelectionResult {
    pub rect: SelectionRect,
    pub mouse_move_dir: VerticalMoveDir,
}

#[derive(Debug)]
struct SelectionState {
    mouse_down_pos: (f64, f64),
    mouse_down_ts: Instant,
    last_click_ts: Option<Instant>,
    last_click_pos: (f64, f64),
    last_check_ts: Instant,
}

pub fn listen<T: 'static + MouseExtTrait + Send>(app: T) {
    let state = Mutex::new(SelectionState {
        mouse_down_pos: (0.0, 0.0),
        mouse_down_ts: Instant::now(),
        last_click_ts: None,
        last_click_pos: (0.0, 0.0),
        last_check_ts: Instant::now(),
    });

    let mut mouse = mouce::Mouse::new();

    let result = mouse.hook(Box::new(move |event| {
        let mut state = state.lock().unwrap();

        match event {
            MouseEvent::AbsoluteMove(..) => {
                app.on_mouse_move();
            }
            MouseEvent::Press(MouseButton::Left) => {
                state.mouse_down_pos = app.get_cursor_position();
                state.mouse_down_ts = Instant::now();
                app.on_mouse_down();
            }
            MouseEvent::Release(MouseButton::Left) => {
                let mut should_check_selection = false;

                let current_mouse_pos = app.get_cursor_position();

                let now = Instant::now();
                let maybe_click = distance(state.mouse_down_pos, current_mouse_pos) < 5.0;

                if maybe_click {
                    if let Some(last_click_ts) = state.last_click_ts {
                        let db_click_max_delay_check_ms = 500;

                        let maybe_double_click = now.duration_since(last_click_ts)
                            < Duration::from_millis(db_click_max_delay_check_ms)
                            && distance(state.last_click_pos, current_mouse_pos) < 5.0;

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

                        let result = text_selection::get_selected_rect();

                        let result = result.map(|rect| {
                            let dir = if current_mouse_pos.1 > state.mouse_down_pos.1 {
                                VerticalMoveDir::Down
                            } else {
                                VerticalMoveDir::Up
                            };

                            return SelectionResult {
                                rect,
                                mouse_move_dir: dir,
                            };
                        });

                        app.on_selection_change(result);

                        log::info!("check selection end");

                        state.last_check_ts = now;
                    }
                } else {
                    app.on_selection_change(None);
                }
            }
            _ => {}
        }
    }));

    if let Err(err) = result {
        log::error!("error is {:?}", err);
    }
}

fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let x = p1.0 - p2.0;
    let y = p1.1 - p2.1;

    return (x * x + y * y).sqrt();
}
