use piston::input::Key;

#[derive(Default)]
pub struct Actions {
    pub move_left: bool,
    pub move_right: bool,
    pub accelerate: bool,
    pub brake: bool,
}

pub struct InputController {
    actions: Actions,
}

impl InputController {
    pub fn new() -> InputController {
        InputController {
            actions: Actions {
                move_left: false,
                move_right: false,
                accelerate: false,
                brake: false,
            },
        }
    }
    pub fn get_actions(&self) -> &Actions {
        &self.actions
    }
    pub fn handle_key_press(&mut self, key: Key) {
        self.handle_key(key, true);
    }
    pub fn handle_key_release(&mut self, key: Key) {
        self.handle_key(key, false);
    }
    fn handle_key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::Left => self.actions.move_left = pressed,
            Key::Right => self.actions.move_right = pressed,
            Key::Up => self.actions.accelerate = pressed,
            Key::Down => self.actions.brake = pressed,
            _ => (),
        }
    }
}
