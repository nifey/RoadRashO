extern crate graphics;

use graphics::types::Color;

const SEGMENT_LENGTH: i32 = 200;
const RUMBLE_LENGTH: i32 = 3;
const DARK_COLOR: Color = [0.2, 0.2, 0.2, 1.0];
const LIGHT_COLOR: Color = [0.8, 0.8, 0.8, 1.0];

struct Coordinates {
    x: i32,
    y: i32,
    z: i32,
}

struct Point {
    world: Coordinates,
    camera: Coordinates,
    screen: Coordinates,
}
impl Point {
    fn project(
        &mut self,
        camera: Coordinates,
        camera_depth: i32,
        width: i32,
        height: i32,
        road_width: i32,
    ) {
        self.camera.x = self.world.x - camera.x;
        self.camera.y = self.world.y - camera.y;
        self.camera.z = self.world.z - camera.z;
        let scale: i32 = camera_depth / self.camera.z;
        self.screen.x = width / 2 + scale * self.camera.x * width / 2;
        self.screen.y = height / 2 + scale * self.camera.y * height / 2;
        self.screen.z = scale * road_width * width / 2;
    }
}

struct Segment {
    index: i32,
    color: Color,
    start: Point,
    end: Point,
}

pub struct Road {
    segments: Vec<Segment>,
}

impl Road {
    pub fn new() -> Self {
        let mut segments: Vec<Segment> = Vec::new();
        for n in 0..100 {
            segments.push(Segment {
                index: n,
                color: match (n / RUMBLE_LENGTH) % 2 {
                    0 => DARK_COLOR,
                    1 => LIGHT_COLOR,
                    _ => DARK_COLOR,
                },
                start: Point {
                    world: Coordinates {
                        x: 0,
                        y: 0,
                        z: n * SEGMENT_LENGTH,
                    },
                    camera: Coordinates { x: 0, y: 0, z: 0 },
                    screen: Coordinates { x: 0, y: 0, z: 0 },
                },
                end: Point {
                    world: Coordinates {
                        x: 0,
                        y: 0,
                        z: n * SEGMENT_LENGTH,
                    },
                    camera: Coordinates { x: 0, y: 0, z: 0 },
                    screen: Coordinates { x: 0, y: 0, z: 0 },
                },
            });
        }
        Road { segments: segments }
    }
    fn find_segment(&self, z: i32) -> &Segment {
        &self.segments[(z / SEGMENT_LENGTH) as usize % self.segments.len()]
    }
}
