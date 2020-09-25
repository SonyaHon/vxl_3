pub struct UserInput {
    pressed_keys: std::collections::BTreeSet<u32>,
}

impl UserInput {
    pub fn add_key(&mut self, key_code: u32) {
        self.pressed_keys.insert(key_code);
    }

    pub fn remove_key(&mut self, key_code: u32) {
        self.pressed_keys.remove(&key_code);
    }

    pub fn is_key_pressed(&self, key_code: u32) -> bool {
        self.pressed_keys.contains(&key_code)
    }
}

impl Default for UserInput {
    fn default() -> Self {
        UserInput {
            pressed_keys: std::collections::BTreeSet::new(),
        }
    }
}
