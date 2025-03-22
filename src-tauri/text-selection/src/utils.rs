use enigo::{Enigo, Key, Keyboard, Settings};

pub fn simulate_esc_key_press() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    enigo.key(Key::Escape, enigo::Direction::Click).unwrap();
    // enigo.key(Key::CapsLock, enigo::Direction::Click).unwrap();
}
