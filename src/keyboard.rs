use std::collections::HashSet;

pub struct Keyboard {
    keys_pressed: HashSet<char>,
}

impl Keyboard {
    pub fn init() -> Keyboard {
        return Keyboard {
            keys_pressed: HashSet::with_capacity(16),
        }
    }
    pub fn add_key_press(&mut self, key: char){
        self.keys_pressed.insert(key);
    }
    
    pub fn set_state(&mut self, keys: HashSet<char>){
        self.keys_pressed = keys.clone();
    }

    pub fn is_valid_key(c: char) -> bool {
        //return true if part of hex keypad.  
        true
    }
}