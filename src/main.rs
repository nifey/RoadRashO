extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;

mod road;

use road::Road;

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

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;
                clear([1.0, 1.0, 1.0, 1.0], g);

                let position: i32 = 0;
            });
        }
        if let Some(args) = e.update_args() {}
    }
}
