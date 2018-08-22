use enigo::{Enigo, KeyboardControllable, Key};

pub struct InputManager {
  enigo: Enigo,
}

impl InputManager {
  pub fn new() -> InputManager {
    InputManager {
      enigo: Enigo::new(),
    }
  }
  
  pub fn press_w(&mut self) {
    self.enigo.key_click(Key::Layout('W'));
  }
}
