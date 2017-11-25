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
        .fullscreen(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(opengl);

    let mut road: Road = Road::new();
    let mut position: f32 = 0.0;
    let mut player_x: f32 = 0.0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::*;
                clear([1.0, 1.0, 1.0, 1.0], g);
                let base_segment: usize = road.find_segment(position);
                let mut max_y: f32 = 768.0;
                for s in road.segments.iter_mut() {
                    if s.index < DRAW_DISTANCE {
                        s.start.project(
                            Coordinates {
                                x: player_x,
                                y: 0.0,
                                z: position,
                            },
                            1024,
                            768,
                        );
                        s.end.project(
                            Coordinates {
                                x: player_x,
                                y: 0.0,
                                z: position,
                            },
                            1024,
                            768,
                        );
                        if s.start.camera.z <= CAMERA_DEPTH || s.end.screen.y >= max_y {
                            continue;
                        }

                        let polygon1: Polygon = Polygon::new(s.color);
                        let vertices =vec![[s.start.screen.x as f64, s.start.screen.y as f64],
                            [
                                (s.start.screen.x + s.start.screen.z) as f64,
                                s.start.screen.y as f64
                            ],
                            [
                                (s.end.screen.x + s.end.screen.z) as f64,
                                s.end.screen.y as f64
                            ],[s.end.screen.x as f64, s.end.screen.y as f64],
[s.start.screen.x as f64, s.start.screen.y as f64]
                        ] ;
                            println!("{:?}",vertices);
/*                        vertices.push([s.start.screen.x as f64, s.start.screen.y as f64]);
                        vertices.push(
                            [
                                (s.start.screen.x + s.start.screen.z) as f64,
                                s.start.screen.y as f64,
                            ],
                        );
                        vertices.push(
                            [
                                (s.end.screen.x + s.end.screen.z) as f64,
                                s.end.screen.y as f64,
                            ],
                        );
                        vertices.push([s.end.screen.x as f64, s.end.screen.y as f64]);
                        vertices.push([s.start.screen.x as f64, s.start.screen.y as f64]);
 */                       let transform = c.transform.trans(0.0, 0.0);
                        polygon(s.color, &vertices[..], transform, g);


                        max_y = s.end.screen.y;
                    }
                }
            });
        }
        if let Some(args) = e.update_args() {}
    }
}
