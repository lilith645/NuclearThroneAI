use enigo::{Enigo, KeyboardControllable, Key, MouseButton};
use enigo::MouseControllable;
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
  
  pub fn press_a(&mut self) {
    self.enigo.key_click(Key::Layout('A'));
  }
  
  pub fn press_s(&mut self) {
    self.enigo.key_click(Key::Layout('S'));
  }
  
  pub fn press_d(&mut self) {
    self.enigo.key_click(Key::Layout('D'));
  }
  
  pub fn press_spacebar(&mut self) {
    self.enigo.key_click(Key::Space);
  }
  
  pub fn move_mouse_to(&mut self, x: i32, y: i32) {
    self.enigo.mouse_move_to(x,y);
  }
  
  pub fn move_mouse_relative(&mut self, x: i32, y: i32) {
    self.enigo.mouse_move_relative(x,y);
  }
  
  pub fn mouse_left_click(&mut self) {
    self.enigo.mouse_click(MouseButton::Left);
  }
  
  pub fn mouse_right_click(&mut self) {
    self.enigo.mouse_click(MouseButton::Right);
  }
}
