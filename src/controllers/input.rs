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
}
