extern crate graphics;

use graphics::types::Color;

const ROAD_WIDTH: f32 = 2000.0;
const SEGMENT_LENGTH: i32 = 200;
const RUMBLE_LENGTH: i32 = 3;
const DARK_COLOR: Color = [0.2, 0.2, 0.2, 1.0];
const LIGHT_COLOR: Color = [0.8, 0.8, 0.8, 1.0];
const LANES: i32 = 3;

pub struct Coordinates {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Point {
    pub world: Coordinates,
    pub camera: Coordinates,
    pub screen: Coordinates,
}

impl Point {
    pub fn project(&mut self, camera: &Coordinates, width: i32, height: i32) {
        self.camera.x = self.world.x - (camera.x * ROAD_WIDTH);
        self.camera.y = self.world.y - camera.y;
        self.camera.z = self.world.z - camera.z;
        let scale: f32 = CAMERA_DEPTH / self.camera.z;
        self.screen.x = width as f32 / 2.0 + (scale * self.camera.x * width as f32 / 2.0);
        self.screen.y = height as f32 / 2.0 - (scale * self.camera.y * height as f32 / 2.0);
        self.screen.z = scale * ROAD_WIDTH * width as f32 / 2.0;
    }
}

pub struct Segment {
    pub index: i32,
    pub color: Color,
    pub start: Point,
    pub end: Point,
}

impl Segment {
    pub fn get_polygon(&self) {}
}

pub struct Road {
    pub segments: Vec<Segment>,
}

impl Road {
    pub fn new() -> Road {
        let mut segments: Vec<Segment> = Vec::new();
        for n in 0..500 {
            segments.push(Segment {
                index: n,
                color: match (n / RUMBLE_LENGTH) % 2 {
                    0 => DARK_COLOR,
                    1 => LIGHT_COLOR,
                    _ => DARK_COLOR,
                },
                start: Point {
                    world: Coordinates {
                        x: 0.0,
                        y: 0.0,
                        z: (n * SEGMENT_LENGTH) as f32,
                    },
                    camera: Coordinates {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    screen: Coordinates {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
                end: Point {
                    world: Coordinates {
                        x: 0.0,
                        y: 0.0,
                        z: ((n + 1) * SEGMENT_LENGTH) as f32,
                    },
                    camera: Coordinates {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    screen: Coordinates {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
            });
        }
        Road { segments: segments }
    }
    pub fn find_segment(&self, z: f32) -> usize {
        (z / SEGMENT_LENGTH as f32) as usize % self.segments.len()
    }
}
