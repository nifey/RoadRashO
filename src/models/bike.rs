use controllers::Actions;

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Straight,
}
const ACCELERATION: f32 = 20.0;
const DECELERATION: f32 = 10.0;
const BRAKE_DECELERATION: f32 = 15.0;
const MAX_SPEED: f32 = 160.0;
const DX: f32 = 0.05;
pub struct Bike {
    x_position: f32,
    z_position: f32,
    speed: f32,
    direction: Direction,
//  color:
}

impl Bike {
    pub fn new(x: f32, z: f32) -> Bike {
        Bike {
            x_position: x,
            z_position: z,
            speed: 0.0,
            direction: Direction::Straight,
        }
    }
    pub fn get_x_position(&self) -> f32 {
        self.x_position
    }
    pub fn get_z_position(&self) -> f32 {
        self.z_position
    }
    pub fn get_direction(&self) -> Direction {
        self.direction
    }
    pub fn update(&mut self, actions: &Actions) {
        self.direction = Direction::Straight;
        if !(actions.move_left && actions.move_right) {
            if actions.move_left {
                self.move_left();
                self.direction = Direction::Left;
            }
            if actions.move_right {
                self.move_right();
                self.direction = Direction::Right;
            }
        }
        self.update_speed(actions.accelerate, actions.brake)
    }
    fn move_left(&mut self) {
        self.x_position -= DX;
        if self.x_position < -1.0 {
            self.x_position = -1.0;
        }
    }
    fn move_right(&mut self) {
        self.x_position += DX;
        if self.x_position > 1.0 {
            self.x_position = 1.0;
        }
    }
    fn update_speed(&mut self, accelerate: bool, brake: bool) {
        if brake {
            self.speed -= BRAKE_DECELERATION;
            if self.speed < 0.0 {
                self.speed = 0.0;
            }
        } else {
            if accelerate {
                self.speed += ACCELERATION;
                if self.speed > MAX_SPEED {
                    self.speed = MAX_SPEED;
                }
            } else {
                self.speed -= DECELERATION;
                if self.speed < 0.0 {
                    self.speed = 0.0;
                }
            }
        }
    }
    fn update_z_position(&mut self) {
        self.z_position += self.speed;
    }
}
