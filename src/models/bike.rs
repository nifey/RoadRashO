use controllers::Actions;

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Straight,
}
const ACCELERATION: f32 = 120.0;
const DECELERATION: f32 = 50.0;
const BRAKE_DECELERATION: f32 = 80.0;
const MAX_SPEED: f32 = 160.0;
const DX: f32 = 0.5;
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
    pub fn get_speed(&self) -> f32 {
        self.speed
    }
    pub fn update(&mut self, actions: &Actions, dt: f32) {
        self.direction = Direction::Straight;
        if !(actions.move_left && actions.move_right) {
            if actions.move_left && self.speed > 0.0 {
                self.move_left(dt);
                self.direction = Direction::Left;
            }
            if actions.move_right && self.speed > 0.0 {
                self.move_right(dt);
                self.direction = Direction::Right;
            }
        }
        self.update_speed(actions.accelerate, actions.brake, dt);
        self.update_z_position(dt);

    }
    fn move_left(&mut self, dt: f32) {
        self.x_position -= DX * dt;
        if self.x_position < -1.0 {
            self.x_position = -1.0;
        }
    }
    fn move_right(&mut self, dt: f32) {
        self.x_position += DX * dt;
        if self.x_position > 1.0 {
            self.x_position = 1.0;
        }
    }
    fn update_speed(&mut self, accelerate: bool, brake: bool, dt: f32) {
        if brake {
            self.speed -= BRAKE_DECELERATION * dt;
            if self.speed < 0.0 {
                self.speed = 0.0;
            }
        } else {
            if accelerate {
                self.speed += ACCELERATION * dt;
                if self.speed > MAX_SPEED {
                    self.speed = MAX_SPEED;
                }
            } else {
                self.speed -= DECELERATION * dt;
                if self.speed < 0.0 {
                    self.speed = 0.0;
                }
            }
        }
    }
    fn update_z_position(&mut self, dt: f32) {
        self.z_position += self.speed * dt;
    }
}
