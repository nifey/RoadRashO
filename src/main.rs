extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;

mod road;

use road::{Road, Coordinates, CAMERA_DEPTH};

const DRAW_DISTANCE: i32 = 300;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("RoadRashO", [1024, 768])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(opengl);

    let mut road: Road = Road::new();
    let length: usize = road.segments.len();
    let mut player: Coordinates = Coordinates {
        x: 0.5,
        y: 0.0,
        z: 0.0,
    };
    let mut camera: Coordinates = Coordinates {
        x: 0.5,
        y: 1000.0,
        z: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::*;
                clear([1.0, 1.0, 1.0, 1.0], g);

                let base_segment: usize = road.find_segment(player.z);
                let mut max_y: f32 = 1024.0;
                for i in 0..DRAW_DISTANCE {
                    let s = &mut road.segments[(base_segment + i as usize) % length];
                    println!("{}", s.index);
                    s.start.project(&camera, 1024, 768);
                    s.end.project(&camera, 1024, 768);
                    if s.start.camera.z <= CAMERA_DEPTH || s.end.screen.y >= max_y {
                        continue;
                    }

                    let vertices = vec![
                        [s.start.screen.x as f64, s.start.screen.y as f64],
                        [
                            (s.start.screen.x + s.start.screen.z) as f64,
                            s.start.screen.y as f64,
                        ],
                        [
                            (s.end.screen.x + s.end.screen.z) as f64,
                            s.end.screen.y as f64,
                        ],
                        [s.end.screen.x as f64, s.end.screen.y as f64],
                        [s.start.screen.x as f64, s.start.screen.y as f64],
                    ];
                    let transform = c.transform.trans(0.0, 0.0);
                    polygon(s.color, &vertices[..], transform, g);


                    max_y = s.end.screen.y;
                }
            });
        }
        if let Some(args) = e.update_args() {
            camera.z += 7000.0 * args.dt as f32;
            player.z = camera.z;
        }
    }
}
