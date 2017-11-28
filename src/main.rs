extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;

mod models;
mod controllers;
//mod view;

use models::Bike;

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

    let b: Bike = Bike::new(0.0, 2.0);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {}
        if let Some(args) = e.update_args() {}
    }
}
