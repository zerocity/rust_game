use ggez::event::Keycode;
use std::collections::HashMap;

pub struct Keyboard(pub HashMap<Keycode, bool>);
impl Keyboard {
    pub fn new() -> Self {
        Keyboard(HashMap::new())
    }

    pub fn is_pressed(&self, btn: Keycode) -> bool {
        match self.0.get(&btn) {
            Some(s) => *s,
            None => false,
        }
    }
}
