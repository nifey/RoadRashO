extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("RoadRashO", [800, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .fullscreen(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    let mut i: f64 = 0.0;
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;
                clear([1.0 - i as f32, i as f32, i as f32, 1.0], g);
            });
        }
        if let Some(args) = e.update_args() {
            i += 0.05 * args.dt;
        }
    }
}
