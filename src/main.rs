extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;

mod models;
mod controllers;
//mod view;

use models::Bike;
use controllers::InputController;

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

    let mut b: Bike = Bike::new(0.0, 2.0);
    let mut input_controller: InputController = InputController::new();
    use models::Direction;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(args) = e.press_args() {
            match args {
                Button::Keyboard(key) => input_controller.handle_key_press(key),
                _ => (),
            }
        }

        if let Some(args) = e.release_args() {
            match args {
                Button::Keyboard(key) => input_controller.handle_key_release(key),
                _ => (),
            }
        }

        if let Some(args) = e.render_args() {
            println!(
                "{} {} {}",
                b.get_speed(),
                b.get_x_position(),
                b.get_z_position()
            );
            match b.get_direction() {
                Direction::Left => println!("<"), 
                Direction::Right => println!(">"), 
                _ => println!("|"),
            }
        }

        if let Some(args) = e.update_args() {
            b.update(input_controller.get_actions(), args.dt as f32);
        }

    }
}
